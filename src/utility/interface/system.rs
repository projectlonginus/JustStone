use crate::utility::interface::utils::SystemInterface;

impl SystemInterface {
    pub fn information() -> String {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        format!(
            "
        [ system information ]

        total memory: {} bytes
        used memory : {} bytes
        total swap  : {} bytes
        used swap   : {} bytes

        System global_cpu_info:  {:?}
        System name              {:?}
        System kernel version:   {:?}
        System OS version:       {:?}
        System host name:        {:?}",
            sys.total_memory(),
            sys.used_memory(),
            sys.total_swap(),
            sys.used_swap(),
            sys.global_cpu_info(),
            sysinfo::System::name(),
            sysinfo::System::kernel_version(),
            sysinfo::System::os_version(),
            sysinfo::System::host_name(),
        )
    }
}