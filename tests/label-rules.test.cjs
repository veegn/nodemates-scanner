const test = require('node:test');
const assert = require('node:assert/strict');
const {
    buildCertificateBadges,
    findCertificateBrand,
    formatCertificateValidity,
    isDomainOrSubdomain,
} = require('../static/label-rules.js');

const NOW = 2_000_000_000;

function labels(row) {
    return buildCertificateBadges({
        origin: '74.125.103.38',
        cert_sans: [],
        cert_chain_trusted: true,
        cert_hostname_match: 'not_applicable',
        cert_self_signed: 'no',
        cert_validation: 'name_mismatch',
        ...row,
    }, NOW);
}

test('recognizes a trusted Google SAN on a Google ASN as direct brand hosting', () => {
    const badges = labels({
        cert_sans: ['*.c.docs.google.com'],
        asn_number: 15169,
        asn_org: 'Google LLC',
    });

    assert.ok(badges.some((badge) => badge.zh === '已知品牌域名'));
    assert.ok(badges.some((badge) => badge.zh === '品牌网络直连'));
    assert.ok(!badges.some((badge) => badge.type === 'danger'));
});

test('does not infer a brand from an attacker-controlled subject string', () => {
    const badges = labels({
        cert_domain: 'CN=google.com.evil.org, O=Google Fake',
        cert_sans: ['google.com.evil.org'],
        asn_org: 'Example Hosting',
    });

    assert.ok(!badges.some((badge) => badge.zh === '已知品牌域名'));
    assert.equal(findCertificateBrand(['google.com.evil.org']), null);
});

test('separates edge, cloud, and unknown third-party hosting without danger labels', () => {
    const edge = labels({ cert_sans: ['docs.google.com'], asn_number: 13335, asn_org: 'Cloudflare, Inc.' });
    const cloud = labels({ cert_sans: ['docs.google.com'], asn_number: 16509, asn_org: 'Amazon.com, Inc.' });
    const thirdParty = labels({ cert_sans: ['docs.google.com'], asn_number: 36352, asn_org: 'HostPapa' });

    assert.ok(edge.some((badge) => badge.zh === 'CDN/边缘承载'));
    assert.ok(cloud.some((badge) => badge.zh === '公有云承载'));
    assert.ok(thirdParty.some((badge) => badge.zh === '第三方网络承载'));
    assert.ok(!thirdParty.some((badge) => badge.type === 'danger'));
});

test('uses current validity timestamps instead of certificate lifetime', () => {
    const row = {
        cert_not_before: NOW - 30 * 86400,
        cert_not_after: NOW + 5 * 86400,
    };
    const badges = labels(row);

    assert.ok(badges.some((badge) => badge.zh === '即将到期'));
    assert.equal(formatCertificateValidity(row, 'zh', NOW), '剩余 5 天');
});

test('marks hostname mismatch and expired certificates as evidence-based risks', () => {
    const badges = labels({
        origin: 'example.com',
        cert_sans: ['other.example'],
        cert_hostname_match: 'mismatched',
        cert_not_after: NOW - 86400,
        cert_chain_trusted: false,
        cert_validation: 'expired',
    });

    assert.ok(badges.some((badge) => badge.zh === '目标域名不匹配' && badge.type === 'danger'));
    assert.ok(badges.some((badge) => badge.zh === '证书已过期' && badge.type === 'danger'));
});

test('domain suffix matching rejects lookalike parent domains', () => {
    assert.equal(isDomainOrSubdomain('c.docs.google.com', 'google.com'), true);
    assert.equal(isDomainOrSubdomain('google.com.evil.org', 'google.com'), false);
});
