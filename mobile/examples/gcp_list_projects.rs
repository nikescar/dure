//! Example: List GCP Projects
//!
//! This example demonstrates how to use the GCP REST API client to list projects.
//!
//! ## Prerequisites
//!
//! 1. You need a valid GCP access token. Get one using:
//!    ```bash
//!    gcloud auth application-default print-access-token
//!    ```
//!
//! 2. Set the token as an environment variable:
//!    ```bash
//!    export GCP_ACCESS_TOKEN="ya29...."
//!    ```
//!
//! ## Usage
//!
//! ```bash
//! # List all projects
//! cargo run --example gcp_list_projects
//!
//! # List projects with a filter
//! cargo run --example gcp_list_projects -- --filter "labels.env:prod"
//! ```
//!
//! ## API Reference
//!
//! https://cloud.google.com/resource-manager/reference/rest/v1/projects/list

use dure::calc::gcp_rest::GcpRestClient;
use std::env;

fn main() -> anyhow::Result<()> {
    // Get access token from environment
    let access_token =
        env::var("GCP_ACCESS_TOKEN").expect("GCP_ACCESS_TOKEN environment variable must be set");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let filter = if args.len() > 2 && args[1] == "--filter" {
        Some(args[2].as_str())
    } else {
        None
    };

    // Create client
    let client = GcpRestClient::new(access_token);

    // List projects
    println!("Listing GCP projects...");
    if let Some(f) = filter {
        println!("Filter: {}", f);
    }
    println!();

    let result = client.list_projects(filter)?;

    // Display results
    if result.projects.is_empty() {
        println!("No projects found.");
    } else {
        println!("Found {} project(s):\n", result.projects.len());

        for (i, project) in result.projects.iter().enumerate() {
            println!("{}. {}", i + 1, project.display_name());
            println!("   Project ID: {}", project.project_id);
            println!("   State: {}", project.state());
            println!("   Active: {}", project.is_active());

            if !project.labels.is_empty() {
                println!("   Labels:");
                for (key, value) in &project.labels {
                    println!("     - {}: {}", key, value);
                }
            }
            println!();
        }

        if let Some(next_token) = &result.next_page_token {
            println!("Next page token: {}", next_token);
            println!("(Pagination not implemented in this example)");
        }
    }

    Ok(())
}
