// @generated automatically by Diesel CLI.

diesel::table! {
    acme_certificates (domain) {
        domain -> Text,
        cert_path -> Text,
        key_path -> Text,
        ca_path -> Text,
        fullchain_path -> Text,
        issued_at -> BigInt,
        expires_at -> BigInt,
        is_valid -> Integer,
    }
}

diesel::table! {
    crypt_keys (device_id) {
        device_id -> Text,
        private_key -> Binary,
        public_key -> Binary,
        created_at -> BigInt,
    }
}

diesel::table! {
    dns_cache (domain, record_type, value) {
        domain -> Text,
        record_type -> Text,
        value -> Text,
        ttl -> BigInt,
        timestamp -> BigInt,
    }
}

diesel::table! {
    hosting (id) {
        id -> BigInt,
        domain -> Text,
        status -> Text,
        domain_registrar -> Nullable<Text>,
        domain_registrar_token -> Nullable<Text>,
        domain_registered -> Integer,
        dns_provider -> Text,
        dns_provider_token -> Nullable<Text>,
        ns_addresses -> Nullable<Text>,
        dns_configured -> Integer,
        vm_provider -> Text,
        vm_provider_token -> Nullable<Text>,
        vm_instance_id -> Nullable<Text>,
        vm_ip_address -> Nullable<Text>,
        vm_ssh_user -> Nullable<Text>,
        vm_ssh_key_path -> Nullable<Text>,
        vm_created -> Integer,
        service_installed -> Integer,
        service_running -> Integer,
        created_at -> BigInt,
        updated_at -> BigInt,
        error_message -> Nullable<Text>,
    }
}

diesel::table! {
    nft_whitelist (ip) {
        ip -> Text,
        description -> Text,
        added_at -> BigInt,
    }
}

diesel::table! {
    sessions (session_id) {
        session_id -> Text,
        domain -> Text,
        session_type -> Text,
        connected_at -> BigInt,
        last_seen -> BigInt,
        request_count -> BigInt,
        remote_addr -> Text,
    }
}

diesel::table! {
    webhook_allow_patterns (id) {
        id -> BigInt,
        pattern -> Text,
        created_at -> BigInt,
    }
}

diesel::table! {
    webhook_config (id) {
        id -> Integer,
        logging_enabled -> Integer,
    }
}

diesel::table! {
    webhook_requests (id) {
        id -> BigInt,
        pattern -> Text,
        path -> Text,
        method -> Text,
        headers -> Text,
        body -> Text,
        remote_addr -> Text,
        received_at -> BigInt,
    }
}

diesel::table! {
    wss_servers (domain) {
        domain -> Text,
        bind_addr -> Text,
        bind_port -> Integer,
        server_id -> Text,
        ping_interval -> Integer,
        idle_timeout -> Integer,
        max_connections -> Integer,
    }
}

diesel::table! {
    wss_sessions (session_id) {
        session_id -> Text,
        domain -> Text,
        connected_at -> BigInt,
        last_seen -> BigInt,
        message_count -> BigInt,
        reconnect_count -> BigInt,
    }
}

diesel::joinable!(wss_sessions -> wss_servers (domain));

diesel::allow_tables_to_appear_in_same_query!(
    acme_certificates,
    crypt_keys,
    dns_cache,
    hosting,
    nft_whitelist,
    sessions,
    webhook_allow_patterns,
    webhook_config,
    webhook_requests,
    wss_servers,
    wss_sessions,
);
