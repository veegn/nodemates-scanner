(function initLabelRules(root, factory) {
    const api = factory();
    if (typeof module === 'object' && module.exports) {
        module.exports = api;
    } else {
        root.LabelRules = api;
    }
}(typeof globalThis !== 'undefined' ? globalThis : this, function createLabelRules() {
    const BRANDS = [
        { name: 'Cloudflare', domains: ['cloudflare.com', 'cloudflare-dns.com', 'workers.dev', 'pages.dev'], asns: [13335, 209242], orgs: ['CLOUDFLARE'] },
        { name: 'Google', domains: ['google.com', 'googleapis.com', 'googleusercontent.com', 'gstatic.com', 'youtube.com', 'ytimg.com'], asns: [15169, 19527, 36040, 396982], orgs: ['GOOGLE'] },
        { name: 'Apple', domains: ['apple.com', 'icloud.com', 'itunes.com', 'mzstatic.com'], asns: [714, 6185], orgs: ['APPLE'] },
        { name: 'Microsoft', domains: ['microsoft.com', 'bing.com', 'live.com', 'windows.net', 'azure.com', 'office.com'], asns: [8075], orgs: ['MICROSOFT'] },
        { name: 'Akamai', domains: ['akamai.com', 'akamaiedge.net', 'akamaihd.net', 'edgesuite.net'], asns: [16625, 20940], orgs: ['AKAMAI'] },
        { name: 'Fastly', domains: ['fastly.com', 'fastly.net', 'fastlylb.net'], asns: [54113], orgs: ['FASTLY'] },
        { name: 'Amazon', domains: ['amazon.com', 'amazonaws.com', 'cloudfront.net', 'awsstatic.com'], asns: [7224, 14618, 16509], orgs: ['AMAZON', 'AWS'] },
    ];

    const EDGE_PROVIDERS = [
        { asns: [13335, 209242], orgs: ['CLOUDFLARE'] },
        { asns: [16625, 20940], orgs: ['AKAMAI'] },
        { asns: [54113], orgs: ['FASTLY'] },
    ];

    const CLOUD_PROVIDERS = [
        { asns: [7224, 14618, 16509], orgs: ['AMAZON', 'AWS'] },
        { asns: [8075], orgs: ['MICROSOFT'] },
        { asns: [15169, 396982], orgs: ['GOOGLE CLOUD', 'GOOGLE'] },
    ];

    function normalizeDnsName(value) {
        return String(value || '').trim().toLowerCase().replace(/^\*\./, '').replace(/\.$/, '');
    }

    function isDomainOrSubdomain(value, suffix) {
        const domain = normalizeDnsName(value);
        const expected = normalizeDnsName(suffix);
        return domain === expected || domain.endsWith(`.${expected}`);
    }

    function normalizedOrg(value) {
        return String(value || '').toUpperCase().replace(/[^A-Z0-9]+/g, ' ').trim();
    }

    function orgMatches(org, aliases) {
        const normalized = normalizedOrg(org);
        return aliases.some((alias) => {
            const expected = normalizedOrg(alias);
            return normalized === expected
                || normalized.startsWith(`${expected} `)
                || normalized.endsWith(` ${expected}`)
                || normalized.includes(` ${expected} `);
        });
    }

    function providerMatches(provider, asnNumber, asnOrg) {
        const asn = Number(asnNumber || 0);
        return provider.asns.includes(asn) || orgMatches(asnOrg, provider.orgs);
    }

    function findCertificateBrand(sans) {
        const dnsNames = Array.isArray(sans)
            ? sans.filter((value) => value && !isIpAddress(value))
            : [];
        return BRANDS.find((brand) => dnsNames.some((name) => (
            brand.domains.some((domain) => isDomainOrSubdomain(name, domain))
        ))) || null;
    }

    function isIpAddress(value) {
        const input = String(value || '');
        return /^\d{1,3}(?:\.\d{1,3}){3}$/.test(input) || input.includes(':');
    }

    function extractCn(subject) {
        const match = String(subject || '').match(/(?:^|,\s*)CN=([^,]+)/i);
        return match ? match[1].trim() : '';
    }

    function addBadge(badges, type, zh, en, evidenceZh, evidenceEn) {
        badges.push({ type, zh, en, evidenceZh, evidenceEn });
    }

    function buildCertificateBadges(row, nowSeconds = Math.floor(Date.now() / 1000)) {
        const badges = [];
        const subject = String(row.cert_domain || '');
        const sans = Array.isArray(row.cert_sans) ? row.cert_sans : [];
        const identities = [...sans, extractCn(subject)].filter(Boolean);
        const subjectUpper = subject.toUpperCase();
        const hostnameStatus = row.cert_hostname_match || 'unknown';
        const validation = row.cert_validation || 'unknown';
        const notBefore = Number(row.cert_not_before || 0);
        const notAfter = Number(row.cert_not_after || 0);

        if (subjectUpper.includes('TRAEFIK DEFAULT CERT') || subjectUpper.includes('KUBERNETES INGRESS')) {
            addBadge(badges, 'warning', '默认证书', 'Default Cert', '证书主题命中已知默认部署证书', 'Subject matches a known deployment default');
        }

        if (identities.some((name) => normalizeDnsName(name) === 'localhost' || normalizeDnsName(name).endsWith('.local'))) {
            addBadge(badges, 'warning', '本地域名证书', 'Local Name Cert', '证书身份包含 localhost 或 .local', 'Certificate identity contains localhost or .local');
        }

        if (identities.some((name) => /(^|[.\s,_=-])(fake|invalid)([.\s,_=-]|$)/i.test(name))) {
            addBadge(badges, 'warning', '测试/占位证书', 'Test/Placeholder', '证书身份包含明确的 fake 或 invalid 标记', 'Certificate identity contains an explicit fake or invalid marker');
        }

        if (row.cert_self_signed === 'verified') {
            addBadge(badges, 'warning', '自签证书', 'Self-Signed', 'Subject 与 Issuer 相同且自身签名验证通过', 'Subject equals Issuer and the self-signature verifies');
        } else if (row.cert_self_signed === 'suspected') {
            addBadge(badges, 'warning', '疑似自颁发', 'Self-Issued?', 'Subject 与 Issuer 相同，但自身签名无法确认', 'Subject equals Issuer but the self-signature could not be confirmed');
        }

        const notYetValid = notBefore > 0 && nowSeconds < notBefore;
        const expired = notAfter > 0 && nowSeconds >= notAfter;
        if (notYetValid) {
            addBadge(badges, 'danger', '尚未生效', 'Not Yet Valid', '当前时间早于证书生效时间', 'Current time is before the certificate validity period');
        } else if (expired) {
            addBadge(badges, 'danger', '证书已过期', 'Expired', '当前时间晚于证书到期时间', 'Current time is after the certificate expiry');
        } else if (notAfter > 0 && notAfter - nowSeconds <= 14 * 86400) {
            addBadge(badges, 'warning', '即将到期', 'Expiring Soon', '证书将在 14 天内到期', 'Certificate expires within 14 days');
        }

        if (!notYetValid && !expired) {
            if (row.cert_chain_trusted) {
                addBadge(badges, 'success', '证书链可信', 'Trusted Chain', '证书可链接到公共受信根', 'Certificate chains to a public trust root');
            } else if (validation === 'untrusted_issuer') {
                addBadge(badges, 'danger', '签发链不可信', 'Untrusted Issuer', '证书无法链接到公共受信根', 'Certificate does not chain to a public trust root');
            } else if (validation === 'bad_signature') {
                addBadge(badges, 'danger', '证书签名无效', 'Bad Signature', '证书签名验证失败', 'Certificate signature validation failed');
            } else if (validation === 'invalid') {
                addBadge(badges, 'danger', '证书验证失败', 'Validation Failed', '证书链验证返回其他错误', 'Certificate chain validation returned an error');
            }
        }

        if (hostnameStatus === 'matched') {
            addBadge(badges, 'success', '目标域名匹配', 'Hostname Match', '输入域名被证书 SAN 覆盖', 'Requested hostname is covered by certificate SANs');
        } else if (hostnameStatus === 'mismatched') {
            addBadge(badges, 'danger', '目标域名不匹配', 'Hostname Mismatch', '输入域名未被证书 SAN 覆盖', 'Requested hostname is not covered by certificate SANs');
        } else if (hostnameStatus === 'unknown' && row.origin && !isIpAddress(row.origin)) {
            addBadge(badges, 'neutral', '域名未验证', 'Hostname Unknown', '证书未提供可用于判断的 SAN', 'Certificate did not provide usable SAN identities');
        }

        const brand = findCertificateBrand(sans);
        if (!brand) return badges;

        addBadge(badges, 'info', '已知品牌域名', 'Known Brand Domain', `SAN 属于 ${brand.name} 的已知域名`, `SAN belongs to a known ${brand.name} domain`);

        const asnNumber = Number(row.asn_number || 0);
        const asnOrg = row.asn_org || '';
        if (!asnNumber && !asnOrg) {
            addBadge(badges, 'neutral', 'ASN 数据缺失', 'ASN Unavailable', '当前 IP 没有可用 ASN 归属数据', 'No ASN attribution data is available for this IP');
        } else if (providerMatches(brand, asnNumber, asnOrg)) {
            addBadge(badges, 'success', '品牌网络直连', 'Brand Network', `ASN 与 ${brand.name} 网络归属一致`, `ASN matches ${brand.name} network ownership`);
        } else if (EDGE_PROVIDERS.some((provider) => providerMatches(provider, asnNumber, asnOrg))) {
            addBadge(badges, 'info', 'CDN/边缘承载', 'CDN/Edge Hosted', '品牌域名由已知 CDN 或边缘网络承载', 'Brand domain is hosted by a known CDN or edge network');
        } else if (CLOUD_PROVIDERS.some((provider) => providerMatches(provider, asnNumber, asnOrg))) {
            addBadge(badges, 'neutral', '公有云承载', 'Cloud Hosted', '品牌域名位于已知公有云网络', 'Brand domain is hosted on a known public cloud');
        } else {
            addBadge(badges, 'neutral', '第三方网络承载', 'Third-Party Hosted', 'ASN 与品牌网络不同，仅表示网络承载关系未知', 'ASN differs from the brand network; this only indicates an unknown hosting relationship');
        }

        return badges;
    }

    function formatCertificateValidity(row, language, nowSeconds = Math.floor(Date.now() / 1000)) {
        const notBefore = Number(row.cert_not_before || 0);
        const notAfter = Number(row.cert_not_after || 0);
        if (!notAfter) return row.cert_validity || '-';
        if (notBefore > 0 && nowSeconds < notBefore) {
            return language === 'zh' ? '尚未生效' : 'Not yet valid';
        }
        if (nowSeconds >= notAfter) {
            const days = Math.floor((nowSeconds - notAfter) / 86400);
            return language === 'zh' ? `已过期 ${days} 天` : `Expired ${days}d ago`;
        }
        const days = Math.ceil((notAfter - nowSeconds) / 86400);
        return language === 'zh' ? `剩余 ${days} 天` : `${days} days left`;
    }

    return {
        buildCertificateBadges,
        findCertificateBrand,
        formatCertificateValidity,
        isDomainOrSubdomain,
        normalizeDnsName,
    };
}));
