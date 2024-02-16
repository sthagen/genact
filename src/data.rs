static BOOTLOG: &str = include_str!("../data/bootlog.txt");
static CFILES: &str = include_str!("../data/cfiles.txt");
static PACKAGES: &str = include_str!("../data/packages.txt");
static COMPOSERS: &str = include_str!("../data/composer.txt");
static SIMCITY: &str = include_str!("../data/simcity.txt");
static BOOT_HOOKS: &str = include_str!("../data/boot_hooks.txt");
static OS_RELEASES: &str = include_str!("../data/os_releases.txt");
static DOCKER_PACKAGES: &str = include_str!("../data/docker_packages.txt");
static DOCKER_TAGS: &str = include_str!("../data/docker_tags.txt");
static ANSIBLE_ROLES: &str = include_str!("../data/ansible_roles.txt");
static ANSIBLE_TASKS: &str = include_str!("../data/ansible_tasks.txt");
static RKHUNTER_CHECKS: &str = include_str!("../data/rkhunter_checks.txt");
static RKHUNTER_ROOTKITS: &str = include_str!("../data/rkhunter_rootkits.txt");
static RKHUNTER_TASKS: &str = include_str!("../data/rkhunter_tasks.txt");
static JULIA_PACKAGES: &str = include_str!("../data/julia.csv");
static TERRAFORM_AWS_RESOURCES: &str = include_str!("../data/terraform_aws_resources.txt");
static TERRAFORM_AZURE_RESOURCES: &str = include_str!("../data/terraform_azure_resources.txt");
static TERRAFORM_GCP_RESOURCES: &str = include_str!("../data/terraform_gcp_resources.txt");
static TERRAFORM_IDS: &str = include_str!("../data/terraform_ids.txt");

lazy_static::lazy_static! {
    pub static ref BOOTLOG_LIST: Vec<&'static str> = BOOTLOG.lines().collect();
    pub static ref CFILES_LIST: Vec<&'static str> = CFILES.lines().collect();
    pub static ref PACKAGES_LIST: Vec<&'static str> = PACKAGES.lines().collect();
    pub static ref COMPOSERS_LIST: Vec<&'static str> = COMPOSERS.lines().collect();
    pub static ref SIMCITY_LIST: Vec<&'static str> = SIMCITY.lines().collect();
    pub static ref BOOT_HOOKS_LIST: Vec<&'static str> = BOOT_HOOKS.lines().collect();
    pub static ref OS_RELEASES_LIST: Vec<&'static str> = OS_RELEASES.lines().collect();
    pub static ref DOCKER_PACKAGES_LIST: Vec<&'static str> = DOCKER_PACKAGES.lines().collect();
    pub static ref DOCKER_TAGS_LIST: Vec<&'static str> = DOCKER_TAGS.lines().collect();
    pub static ref ANSIBLE_ROLES_LIST: Vec<&'static str> = ANSIBLE_ROLES.lines().collect();
    pub static ref ANSIBLE_TASKS_LIST: Vec<&'static str> = ANSIBLE_TASKS.lines().collect();
    pub static ref RKHUNTER_CHECKS_LIST: Vec<&'static str> = RKHUNTER_CHECKS.lines().collect();
    pub static ref RKHUNTER_ROOTKITS_LIST: Vec<&'static str> = RKHUNTER_ROOTKITS.lines().collect();
    pub static ref RKHUNTER_TASKS_LIST: Vec<&'static str> = RKHUNTER_TASKS.lines().collect();
    pub static ref TERRAFORM_AWS_RESOURCES_LIST: Vec<&'static str> = TERRAFORM_AWS_RESOURCES.lines().collect();
    pub static ref TERRAFORM_AZURE_RESOURCES_LIST: Vec<&'static str> = TERRAFORM_AZURE_RESOURCES.lines().collect();
    pub static ref TERRAFORM_GCP_RESOURCES_LIST: Vec<&'static str> = TERRAFORM_GCP_RESOURCES.lines().collect();
    pub static ref TERRAFORM_IDS_LIST: Vec<&'static str> = TERRAFORM_IDS.lines().collect();
    pub static ref JULIA_PACKAGES_LIST: Vec<crate::modules::julia::Package<'static>> = JULIA_PACKAGES
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let name = split.next().unwrap_or("Revise");
            let id = split.next().unwrap_or("295af30f");
            let versions = split.collect();
            crate::modules::julia::Package { name, id, versions }
        })
        .collect();

}

pub static EXTENSIONS_LIST: &[&str] = &[
    "gif", "mkv", "webm", "mp4", "html", "php", "md", "png", "jpg", "opus", "ogg", "mp3", "flac",
    "iso", "zip", "rar", "tar.gz", "tar.bz2", "tar.xz", "tar.zstd", "deb", "rpm", "exe",
];

pub static COMPRESSION_FORMATS_LIST: &[&str] =
    &["gzip", "bzip2", "lzma", "xz", "lzop", "lz4", "zstd"];
