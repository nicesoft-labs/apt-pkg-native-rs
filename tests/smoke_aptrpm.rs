use apt_pkg_native::Cache;

#[cfg(feature = "niceos-apt-rpm")]
#[test]
fn smoke_aptrpm_cache() {
    let mut cache = Cache::get_singleton();

    let mut iter = cache.iter();
    let mut seen = 0;

    while let Some(pkg) = iter.next() {
        let _name = pkg.name();
        let _arch = pkg.arch();
        let _current = pkg.current_version();
        let _candidate = pkg.candidate_version();

        if let Some(ver) = pkg.versions().next() {
            let _version = ver.version();
            let _ver_arch = ver.arch();
            let _source_pkg = ver.source_package();
            let _source_ver = ver.source_version();
        }

        seen += 1;
        if seen >= 10 {
            break;
        }
    }

    assert!(seen >= 10, "expected at least 10 packages");
}
