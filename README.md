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

This method leverages the GitHub CLI to verify attestations, especially useful when the attestation is pushed to the GitHub repository.

1.  **Install GitHub CLI:**

    Follow the official installation guide: [GitHub CLI Installation](https://cli.github.com/manual/installation)

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
    Verified signature for docker.io/your-username/rust-sigstore-test:vX.Y.Z
    ```

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