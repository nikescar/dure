(introduction)

Dure is a distributed e-commerce platform that empowers small shop owners with complete control over their online stores. Built with Rust and egui, it provides a modern solution for running e-commerce operations without relying on traditional centralized server infrastructure.

(inspiration)

The name "Dure" comes from the Korean word "doorae," which means a community helping each other. This reflects our core philosophy: enabling shop owners to build sustainable businesses through distributed technology and community cooperation.

(core features)

Dure provides comprehensive e-commerce capabilities through five key components:

**Identity Management**: Secure authentication using private/public key pairs, Firebase/Supabase integration, and attestation for WASM/EGUI apps with GitHub Sigstore.

**Guest Front (WASM)**: Customers can browse products, manage carts, and complete purchases with minimal identity requirements.

**Store Front (WASM)**: Shop owners can display products, manage listings, and integrate payment gateways like Portone and KakaoPay.

**Hosting Management (EGUI)**: Complete infrastructure control including DNS management (octodns), database setup (Firebase, Supabase), and site deployment.

**Store Management (EGUI)**: Full business operations including promotions, products, orders, shipments, accounts, and shared listings with other stores.

(technical architecture)

Dure is built on a robust, multi-platform foundation:

- **Language**: Rust (2021 edition) for performance and reliability
- **UI Framework**: egui + eframe with Material3 design
- **Platform Support**: Desktop (Linux, Windows, macOS), Android, and WASM
- **Database**: Diesel ORM with SQLite for all platforms, optional PostgreSQL for desktop
- **Async Runtime**: asupersync for efficient I/O operations

The architecture supports three deployment models: Desktop client (EGUI with system tray), Mobile client (Android with native-activity), and Web client (WASM for guest and store fronts).

(hosting flexibility)

Shop owners can choose their preferred infrastructure:

- **Cloud Platforms**: GCP, Firebase, or Supabase
- **DNS Providers**: Cloudflare, Porkbun, DuckDNS, or Google Cloud DNS
- **Payment Gateways**: Portone and KakaoPay integration
- **Self-Hosting**: Option to deploy on cafe24 or custom VPS

This flexibility means shop owners maintain control over their data and infrastructure choices.

(key advantages)

**Zero Transaction Fees**: Unlike traditional platforms that charge 2% or more per transaction, Dure does not charge transaction fees. Shop owners keep their full revenue.

**Distributed Architecture**: Each shop owner maintains their own database on their chosen cloud service, with frequently accessed content distributed via CDN. This approach provides better scalability and reduces infrastructure costs.

**Data Ownership**: Shop owners have complete control over their customer data, product information, and business analytics.

(content delivery)

Dure provides efficient content delivery through distributed infrastructure. Shop owners store their content on their own CDN, ensuring fast access for customers worldwide. Product listings and updates are synchronized through websocket connections, enabling real-time communication between stores and customers.

(security)

Dure implements multiple layers of security:

**Authentication**: Supports Passkey and FIDO2 authentication for secure client access, with ES256 (ECDSA P-256 with SHA-256) encryption.

**Site-to-Site Communication**: Uses ChaCha20-Poly1305 encryption for secure communication between stores.

**DDoS Protection**: Includes fail2ban integration for automated protection against malicious requests.

**Access Control**: Whitelisted host connections ensure only authorized services can communicate with your store.

**Attestation**: GitHub Sigstore integration provides verification for WASM and EGUI applications.

(customer support)

Dure leverages modern AI services to provide efficient customer support. Shop owners can integrate AI assistants to help resolve customer inquiries and streamline communication, reducing operational overhead while maintaining quality service.

(comparison with major platforms)

| Platform | Hosting | Transaction Fees | Setup Time | Best For |
|----------|---------|------------------|------------|----------|
| **Dure** | Distributed (GCP/Firebase/Supabase) | 0% | Hours | Small shops, full control |
| **Shopify** | Fully managed | 2% | 1-2 days | Quick launch, scalability |
| **Wix** | Fully managed | 0% (limited gateways) | Hours | Beginners, small catalogs |
| **Magento** | Self-hosted or Cloud | 0% | Weeks | Enterprise, large catalogs |

Dure provides the autonomy of self-hosting with the convenience of modern cloud platforms, making it ideal for shop owners who want full control without transaction fees.

(current status)

The Dure platform is actively under development with multiple components already in testing phase. The project features:

- Desktop clients for Linux, Windows, and macOS
- Android mobile application
- WASM-based web frontends for guest and store operations
- Comprehensive CLI and GUI interfaces
- Integration with major cloud platforms and payment gateways

(get started)

To learn more about Dure and join our community:
- Visit the GitHub repository for documentation and source code
- Explore the demo application at dure.app
- Check GitHub Actions for the latest runnable binaries
- Join the community to contribute and provide feedback

(thank you)

Thank you for your interest in Dure. Together, we can empower small shop owners with distributed e-commerce technology.

