use ipnet::IpNet;
use std::{net::IpAddr, str::FromStr};

use crate::{models::SystemSettings, scanner::is_internal_ip};

const MAX_TARGET_IPS: usize = 65_536;

#[derive(Debug)]
pub struct ResolvedTarget {
    pub ips: Vec<IpAddr>,
    pub origins: Vec<String>,
    pub display_target: String,
}

pub async fn resolve_target(
    target: &str,
    settings: &SystemSettings,
) -> Result<ResolvedTarget, String> {
    let clean_target = clean_target(target);
    let mut ips = Vec::new();
    let mut origins = Vec::new();
    let mut display_target = clean_target.clone();

    if let Ok(net) = IpNet::from_str(&clean_target) {
        validate_cidr_size(net, settings)?;
        append_public_hosts(net, &mut ips, &mut origins, |ip| ip.to_string())?;
    } else if let Ok(ip) = IpAddr::from_str(&clean_target) {
        let net = expand_ip_to_subnet(ip, settings)?;
        display_target = net.to_string();
        append_public_hosts(net, &mut ips, &mut origins, |ip| ip.to_string())?;
    } else {
        resolve_domain_target(&clean_target, &mut ips, &mut origins).await?;
    }

    if ips.is_empty() {
        return Err("Error: No public IPs found".into());
    }

    Ok(ResolvedTarget {
        ips,
        origins,
        display_target,
    })
}

fn clean_target(target: &str) -> String {
    let mut clean_target = target.trim().to_string();
    if let Some(rest) = clean_target.strip_prefix("http://") {
        clean_target = rest.to_string();
    } else if let Some(rest) = clean_target.strip_prefix("https://") {
        clean_target = rest.to_string();
    }
    if let Some((_, rest)) = clean_target.split_once('@') {
        clean_target = rest.to_string();
    }
    if let Some(rest) = clean_target.strip_prefix('[')
        && let Some(end) = rest.find(']')
    {
        return rest[..end].to_string();
    }
    if IpNet::from_str(&clean_target).is_ok() {
        return clean_target;
    }
    if let Some(idx) = clean_target.find(['/', '?', '#']) {
        clean_target = clean_target[..idx].to_string();
    }
    clean_target
}

fn validate_cidr_size(net: IpNet, settings: &SystemSettings) -> Result<(), String> {
    match net {
        IpNet::V4(net_v4) if net_v4.prefix_len() < settings.max_cidr_ipv4 => Err(format!(
            "Error: CIDR too large (max /{})",
            settings.max_cidr_ipv4
        )),
        IpNet::V6(net_v6) if net_v6.prefix_len() < settings.max_cidr_ipv6 => Err(format!(
            "Error: CIDR too large (max /{})",
            settings.max_cidr_ipv6
        )),
        _ => Ok(()),
    }
}

fn expand_ip_to_subnet(ip: IpAddr, settings: &SystemSettings) -> Result<IpNet, String> {
    let prefix_len = match ip {
        IpAddr::V4(_) => settings.max_cidr_ipv4,
        IpAddr::V6(_) => settings.max_cidr_ipv6,
    };
    IpNet::new(ip, prefix_len)
        .map(|net| net.trunc())
        .map_err(|_| "Error: Failed to expand IP into subnet".to_string())
}

fn append_public_hosts<F>(
    net: IpNet,
    ips: &mut Vec<IpAddr>,
    origins: &mut Vec<String>,
    origin: F,
) -> Result<(), String>
where
    F: Fn(IpAddr) -> String,
{
    for ip in net.hosts() {
        if !is_internal_ip(&ip) {
            if ips.len() >= MAX_TARGET_IPS {
                return Err(format!(
                    "Error: Target too large (max {} public IPs)",
                    MAX_TARGET_IPS
                ));
            }
            ips.push(ip);
            origins.push(origin(ip));
        }
    }
    Ok(())
}

async fn resolve_domain_target(
    clean_target: &str,
    ips: &mut Vec<IpAddr>,
    origins: &mut Vec<String>,
) -> Result<(), String> {
    let domain = normalize_domain_origin(clean_target);
    if domain.is_empty() {
        return Err("Error: Failed to resolve domain".to_string());
    }

    let resolved = tokio::net::lookup_host((domain.as_str(), 443))
        .await
        .map_err(|_| "Error: Failed to resolve domain".to_string())?;

    let mut addrs: Vec<_> = resolved.map(|addr| addr.ip()).collect();
    addrs.sort();
    addrs.dedup();
    let total_resolved = addrs.len();
    for ip in addrs {
        if !is_internal_ip(&ip) {
            ips.push(ip);
            origins.push(domain.clone());
        }
    }
    if ips.is_empty() && total_resolved > 0 {
        return Err("Error: DNS resolved to internal/fake IPs. If you are using a proxy, please disable fake-ip.".into());
    }

    Ok(())
}

fn normalize_domain_origin(clean_target: &str) -> String {
    let mut domain = clean_target.to_string();
    if let Some(idx) = domain.rfind(':') {
        domain = domain[..idx].to_string();
    }
    domain.trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_settings() -> SystemSettings {
        SystemSettings {
            concurrency_limit: 50,
            max_cidr_ipv4: 24,
            max_cidr_ipv6: 120,
            cooldown_days: 30,
            allowed_ports: "443".into(),
        }
    }

    #[tokio::test]
    async fn single_ipv4_target_expands_to_configured_subnet() {
        let resolved = resolve_target("8.8.8.8", &test_settings())
            .await
            .expect("public IPv4 should resolve to its subnet");

        assert_eq!(resolved.display_target, "8.8.8.0/24");
        assert_eq!(resolved.ips.len(), 254);
        assert_eq!(resolved.origins.len(), resolved.ips.len());
        assert!(resolved.ips.iter().any(|ip| ip.to_string() == "8.8.8.8"));
        assert!(resolved.origins.iter().any(|origin| origin == "8.8.8.8"));
    }

    #[tokio::test]
    async fn explicit_cidr_keeps_requested_target() {
        let resolved = resolve_target("8.8.8.0/30", &test_settings())
            .await
            .expect("small public CIDR should resolve");

        assert_eq!(resolved.display_target, "8.8.8.0/30");
        assert_eq!(resolved.ips.len(), 2);
    }

    #[tokio::test]
    async fn explicit_large_cidr_is_rejected_before_full_expansion() {
        let mut settings = test_settings();
        settings.max_cidr_ipv6 = 64;

        let err = resolve_target("2001:4860::/64", &settings)
            .await
            .expect_err("oversized target should be rejected");

        assert!(err.contains("Target too large"));
    }

    #[test]
    fn domain_origin_strips_port_for_sni() {
        assert_eq!(normalize_domain_origin("example.com:8443"), "example.com");
        assert_eq!(normalize_domain_origin("example.com"), "example.com");
    }

    #[test]
    fn clean_target_strips_url_noise() {
        assert_eq!(
            clean_target("https://user@example.com:8443/path?a=b#c"),
            "example.com:8443"
        );
        assert_eq!(
            clean_target("http://[2001:4860::8888]:443/dns-query"),
            "2001:4860::8888"
        );
    }
}
