# Versioning and Releasing

This document outlines the process for creating a new release of the application.

The project is set up with a GitHub Actions workflow that automates the process of building the application and creating a GitHub Release with the compiled `.exe` file.

## How to Create a New Release

To trigger the workflow and create a new release, follow these steps:

1.  **Commit your changes**: Make sure all your code changes are committed to the main branch.

2.  **Tag a new version**: Create a new Git tag for your release. The tag must start with `v` (e.g., `v1.0.0`, `v0.1.1`).

    ```bash
    git tag vX.Y.Z
    ```
    Replace `X.Y.Z` with the new version number.

3.  **Push the tag to GitHub**: Push the tag to the remote repository to trigger the release workflow.

    ```bash
    git push origin vX.Y.Z
    ```

After pushing the tag, you can navigate to the **Actions** tab in your GitHub repository to monitor the workflow's progress. Once it completes successfully, a new release will be published in the **Releases** section of your repository, and the compiled `.exe` will be available for download as a release asset.
