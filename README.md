# rust-sigstore-test

This project demonstrates how to set up a Rust application for build provenance using Sigstore, specifically for Docker container images.

## How Sigstore Verification is Configured

Sigstore verification for this repository is integrated into the GitHub Actions workflow located at `.github/workflows/release.yml`. This workflow automates the following:

1.  **Docker Image Build and Push:** It builds the Rust application into a Docker image and pushes it to Docker Hub.
2.  **Artifact Attestation Generation:** It generates a Sigstore attestation for the pushed Docker image, linking the image to its build process (provenance).
3.  **GitHub Release:** It creates a GitHub Release, including the SHA256 digest of the Docker image and a link to verify its attestation on Sigstore.

**Key Configuration Points:**

*   **`Dockerfile`:** Your application must be containerized using a `Dockerfile` at the root of your repository. This allows the GitHub Actions workflow to build a Docker image.
*   **GitHub Variables:** You need to configure the following GitHub Variables in your repository settings (`Settings > Variables > Actions`):
    *   `DOCKERHUB_USERNAME`: Your Docker Hub username.
*   **GitHub Secrets:** You need to configure the following GitHub Secrets in your repository settings (`Settings > Secrets and variables > Actions`):
    *   `DOCKERHUB_TOKEN`: A Docker Hub access token with push permissions.
*   **Workflow Permissions:** The `release.yml` workflow requires specific permissions to write attestations and interact with the OIDC token:
    ```yaml
    permissions:
      attestations: write
      id-token: write
      contents: write
      packages: write
    ```

## Verifying the Docker Image Attestation

This method leverages the GitHub CLI to verify attestations, especially useful when the attestation is pushed to the GitHub repository.

1.  **Install GitHub CLI:**

    Follow the official installation guide: [GitHub CLI Installation](https://github.com/cli/cli#installation)

2.  **Verify the Attestation:**

    Replace `your-username` with your GitHub username and `vX.Y.Z` with the specific tag of the image you want to verify (e.g., `v1.0.0`).

    ```bash
    gh attestation verify oci://docker.io/your-username/rust-sigstore-test:vX.Y.Z -R your-username/rust-sigstore-test
    ```

    *   The `oci://` prefix indicates that the image is in an OCI registry.
    *   The `-R` flag specifies the GitHub repository where the attestation is stored.

3.  **Example Successful Output:**

    A successful verification will show output similar to this:

    ```
    gh attestation verify oci://docker.io/mondaylord/rust-sigstore-test:v1.0.1 -R mondaylord/rust-sigstore-test
    Loaded digest sha256:0cdb3fbefe78c7a2a0c1044e4fd174a7f9aa1bc5e8db16040087c238d62f0d45 for oci://docker.io/mondaylord/rust-sigstore-test:v1.0.1
    Loaded 1 attestation from GitHub API

    The following policy criteria will be enforced:
    - Predicate type must match:................ https://slsa.dev/provenance/v1
    - Source Repository Owner URI must match:... https://github.com/mondaylord
    - Source Repository URI must match:......... https://github.com/mondaylord/rust-sigstore-test
    - Subject Alternative Name must match regex: (?i)^https://github.com/mondaylord/rust-sigstore-test/
    - OIDC Issuer must match:................... https://token.actions.githubusercontent.com

    ✓ Verification succeeded!

    The following 1 attestation matched the policy criteria

    - Attestation #1
      - Build repo:..... mondaylord/rust-sigstore-test
      - Build workflow:. .github/workflows/release.yml@refs/tags/v1.0.1
      - Signer repo:.... mondaylord/rust-sigstore-test
      - Signer workflow: .github/workflows/release.yml@refs/tags/v1.0.1
    ```

## Deploying on dstack: Exposing Required Endpoints

To deploy an application on the dstack platform, the application must expose specific HTTP endpoints. These endpoints allow the dstack service to retrieve TEE attestation data, which is crucial for verifying the integrity and identity of the running application.

Specifically, dstack visualizer calls these endpoints when calculating the `app-compose` hash. The information gathered is used to ensure that the calculated `RTMR3` (Runtime-extendable Measurement Register 3) value matches the expected value in the repository (including the docker image digest and docker compose), thus confirming the application's integrity.

This repository provides a reference implementation of the required endpoints. Developers should adapt this example for their own applications.

### Required Endpoints

Your application must implement the following two endpoints:

1.  **`GET /quote`**: This endpoint must return a JSON object containing the TEE quote (`quote`) and the event log (`event_log`).
2.  **`GET /info`**: This endpoint must return a JSON object containing the `app_compose` information from the instance's TCB (Trusted Computing Base) info.

### Reference Implementation

The code in `src/main.rs` demonstrates how to implement these endpoints using `axum` and the `dstack-sdk`. You can run this example to see it in action.

#### Prerequisites

For local testing, you need to run the dstack simulator.

1.  **Clone the dstack repository and run the simulator:**
    ```bash
    git clone https://github.com/Dstack-TEE/dstack.git
    cd dstack/sdk/simulator
    ./build.sh
    ./dstack-simulator
    ```

2.  **Set the environment variable:**
    In a separate terminal, set the endpoint for the simulator.
    ```bash
    export DSTACK_SIMULATOR_ENDPOINT=http://localhost:8000
    ```

#### Running the Example

With the simulator running, you can start the example server from the root of this project:

```bash
cargo run
```

You can then test the endpoints:

```bash
# Test the /quote endpoint
curl http://localhost:8000/quote

# Test the /info endpoint
curl http://localhost:8000/info
```

## Best Practice: Using Image Digests in `docker-compose.yml`

For enhanced security and to ensure immutable infrastructure, it is a best practice to reference Docker images using their SHA256 digest instead of tags (like `:latest` or `:v1.0.0`). This guarantees that you are always running the exact version of the image you intend to, protecting against tag mutability where a tag can be overwritten with a different image.

### How to Use an Image Digest

1.  **Find the Image Digest:** After your image is built and pushed by a CI/CD pipeline (like the one in this repository's GitHub Actions), you can find its SHA256 digest in the GitHub Release notes or in your container registry (e.g., Docker Hub).

2.  **Update `docker-compose.yml`:** Open your `docker-compose.yml` file and update the `image` field to use the digest.

    ```yaml
    services:
      app:
        # Replace with your actual image and digest
        image: docker.io/your-username/rust-sigstore-test@sha256:<your-image-digest>
    ```

3.  **Run the Application:**

    ```bash
    docker compose up
    ```