# rust-sigstore-test

This project demonstrates how to set up a Rust application for build provenance using Sigstore, specifically for Docker container images.

## How Sigstore Verification is Configured

Sigstore verification for this repository is integrated into the GitHub Actions workflow located at `.github/workflows/release.yml`. This workflow automates the following:

1.  **Docker Image Build and Push:** It builds the Rust application into a Docker image and pushes it to Docker Hub.
2.  **Artifact Attestation Generation:** It generates a Sigstore attestation for the pushed Docker image, linking the image to its build process (provenance).
3.  **GitHub Release:** It creates a GitHub Release, including the SHA256 digest of the Docker image and a link to verify its attestation on Sigstore.

**Key Configuration Points:**

*   **`Dockerfile`:** Your application must be containerized using a `Dockerfile` at the root of your repository. This allows the GitHub Actions workflow to build a Docker image.
*   **GitHub Secrets:** You need to configure the following GitHub Secrets in your repository settings (`Settings > Secrets and variables > Actions`):
    *   `DOCKERHUB_USERNAME`: Your Docker Hub username.
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

To verify the provenance of the Docker image using Sigstore's `cosign` tool:

1.  **Install `cosign`:**

    Follow the official installation guide: [Sigstore Cosign Installation](https://docs.sigstore.dev/cosign/installation/)

2.  **Pull the Docker Image:**

    Replace `your-username` with your Docker Hub username and `vX.Y.Z` with the specific tag of the image you want to verify (e.g., `v0.2.5`).

    ```bash
    docker pull docker.io/your-username/rust-sigstore-test:vX.Y.Z
    ```

3.  **Verify the Attestation:**

    Use `cosign verify` to check the image's signature and provenance. This command will verify that the image was built by the specified GitHub Actions workflow and has not been tampered with.

    ```bash
    cosign verify \
      --certificate-identity "https://github.com/your-username/rust-sigstore-test/.github/workflows/release.yml@refs/tags/vX.Y.Z" \
      --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
      docker.io/your-username/rust-sigstore-test:vX.Y.Z
    ```

    *   Replace `your-username` with your Docker Hub username.
    *   Replace `vX.Y.Z` with the image tag (e.g., `v0.2.5`).
    *   The `--certificate-identity` specifies the GitHub Actions workflow that built the image.
    *   The `--certificate-oidc-issuer` specifies the OIDC issuer for GitHub Actions.

## Using `docker-compose.yml`

The `docker-compose.yml` file demonstrates how to reference the Docker image using its SHA256 digest, which is a best practice for ensuring immutability and security.

1.  **Get the Image Digest:**

    After a successful GitHub Actions release workflow run, the image digest (SHA256) will be available in the GitHub Release notes. Copy this digest.

2.  **Update `docker-compose.yml`:**

    Open the `docker-compose.yml` file and replace `<your-image-digest>` with the actual SHA256 digest you copied.

    ```yaml
    services:
      app:
        image: docker.io/your-username/rust-sigstore-test@sha256:<your-image-digest>
    ```

    Remember to replace `your-username` with your Docker Hub username.

3.  **Run the Application:**

    ```bash
    docker compose up
    ```