use std::fs::{self, File};
use std::io::Write;
use dialoguer::{Input, Select};

fn main() {
    // Ask for project name
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact_text()
        .unwrap();

    // Ask for Odoo version
    let odoo_versions = vec!["18", "17", "16", "15", "14", "13"];
    let odoo_version_index = Select::new()
        .with_prompt("Select Odoo version")
        .items(&odoo_versions)
        .interact()
        .unwrap();
    let odoo_version = odoo_versions[odoo_version_index];

    // Map Odoo version to PostgreSQL version
    let postgres_version = match odoo_version {
        "17" | "18" => "16",
        _ => "13",
    };

    // Create folder structure
    let folders = vec!["config", "addons", "custom_addons", "data"];
    for folder in folders {
        let path = format!("{}/{}", project_name, folder);
        fs::create_dir_all(&path).expect("Failed to create directory");
    }

    // Generate docker-compose.yml
    let docker_compose_content = format!(
        r#"version: '3'
services:
  web:
    image: odoo:{}
    ports:
      - "8069:8069"
    volumes:
      - ./config:/etc/odoo
      - ./addons:/mnt/extra-addons
      - ./custom_addons:/mnt/custom-addons
      - ./data:/var/lib/odoo
    depends_on:
      - postgres
    environment:
      - HOST=db
      - USER=odoo
      - PASSWORD=odoo
    restart: always

  postgres:
    image: postgres:{}
    environment:
      - POSTGRES_DB=postgres
      - POSTGRES_PASSWORD=odoo
      - POSTGRES_USER=odoo
    volumes:
      - ./data/pgdata:/var/lib/postgresql/data
    restart: always

volumes:
  odoo-web-data:
  postgres-data:
"#,
        odoo_version, postgres_version
    );

    let docker_compose_path = format!("{}/docker-compose.yml", project_name);
    let mut file = File::create(&docker_compose_path).expect("Failed to create file");
    file.write_all(docker_compose_content.as_bytes())
        .expect("Failed to write to file");

    // Generate odoo.conf
    let odoo_conf_content = format!(
        r#"[options]
addons_path = /mnt/extra-addons,/mnt/custom-addons
data_dir = /var/lib/odoo
admin_passwd = admin
db_host = postgres
db_port = 5432
db_user = odoo
db_password = odoo
"#
    );

    let odoo_conf_path = format!("{}/config/odoo.conf", project_name);
    let mut odoo_conf_file = File::create(&odoo_conf_path).expect("Failed to create odoo.conf");
    odoo_conf_file
        .write_all(odoo_conf_content.as_bytes())
        .expect("Failed to write to odoo.conf");

    println!(
        "Project '{}' with Odoo {} and PostgreSQL {} has been created successfully!",
        project_name, odoo_version, postgres_version
    );
}
