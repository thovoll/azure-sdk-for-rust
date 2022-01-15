use azure_core::prelude::*;
use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;
use chrono::Utc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = create_data_lake_client().await.unwrap();

    let file_system_name = format!("azurerustsdk-datalake-example01-{}", Utc::now().timestamp());
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().into_future().await?;
    println!("create file system response == {:?}\n", create_fs_response);

    let file_path = "some/path/example-file.txt";

    println!("creating file '{}'...", file_path);
    let create_file_response = file_system_client
        .create_file(Context::default(), file_path, FileCreateOptions::default())
        .await?;
    println!("create file response == {:?}\n", create_file_response);

    println!("creating file '{}' if not exists...", file_path);
    let create_file_if_not_exists_result = file_system_client
        .create_file_if_not_exists(Context::default(), file_path)
        .await;
    println!(
        "create file result (should fail) == {:?}\n",
        create_file_if_not_exists_result
    );

    println!("creating file '{}' (overwrite)...", file_path);
    let create_file_response = file_system_client
        .create_file(Context::default(), file_path, FileCreateOptions::default())
        .await?;
    println!("create file response == {:?}\n", create_file_response);

    println!("deleting file '{}'...", file_path);
    let delete_file_response = file_system_client
        .delete_file(Context::default(), file_path, FileDeleteOptions::default())
        .await?;
    println!("delete_file file response == {:?}\n", delete_file_response);

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().into_future().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

async fn create_data_lake_client() -> Result<DataLakeClient, Box<dyn Error + Send + Sync>> {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    Ok(DataLakeClient::new(
        StorageSharedKeyCredential::new(account_name, account_key),
        None,
    ))
}
