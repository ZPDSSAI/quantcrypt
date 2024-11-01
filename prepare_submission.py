# Run this script to automatically zip the correct certificates for submission
# Note: The certficates shall be generated by running the rust testcases as follows:
# Part 1: Generate non-ipd certificates
# cargo test --release  
# Part 1: Generate ipd certificates
# cargo test --release --features ipd  

import os 
import zipfile
import shutil

"""
    Funnction to prepare submission zip with specified version.

    Version should be either r3 or r4.

    For the current version on PQC hackerthon:
    - ML-KEM composites are taken from the IPD version;
    - Pure KEM for both IPD and non-IPD are kept;
    - SLH algorithms are only available in non-IPD.

    Essentially the submission contains:
    - Pure KEM for both ipd (oid = 1.3.xxx) and non-ipd (oid = 2.16.xxx)
    - ML-KEM composites from ipd
    - SLH from non-ipd

"""
def extract_correct_certs(version):
    assert version in ["r3", "r4"], "Unknown submission version"
    cert_format = "pem" if version == "r3" else "der"

    cert_path = f"./artifacts/{version}_certs"
    assert os.path.exists(cert_path), "Certificates not generated, please run Rust test cases first"

    submission_dir = "./artifacts/submission"
    os.makedirs(submission_dir, exist_ok=True)

    # Directory to temporarily hold the r3 certs
    artifacts_certs = os.path.join(submission_dir, f"artifacts_{version}_certs")
    os.makedirs(artifacts_certs, exist_ok=True)
   
    ipd_certs_path = os.path.join(cert_path, "ipd")
    non_ipd_certs_path = os.path.join(cert_path, "non-ipd")
    # Copy files from ipd that end with .der
    for cert in os.listdir(ipd_certs_path):
        if cert.endswith(f".{cert_format}"):
            shutil.copy(os.path.join(ipd_certs_path, cert), artifacts_certs)

    # Copy files from non-ipd only if they do not exist in the r3_certs already
    for cert in os.listdir(non_ipd_certs_path):
        cert_in_r3 = os.path.join(artifacts_certs, cert)
        if not os.path.exists(cert_in_r3) and cert.endswith(f".{cert_format}"):
            shutil.copy(os.path.join(non_ipd_certs_path, cert), artifacts_certs)

    # Zip all files in artifacts_certs for correct version
    zip_filename = os.path.join(submission_dir, f"artifacts_{version}_certs.zip")
    with zipfile.ZipFile(zip_filename, 'w', zipfile.ZIP_DEFLATED) as zipf:
        for root, dirs, files in os.walk(artifacts_certs):
            for file in files:
                full_path = os.path.join(root, file)
                relative_path = os.path.relpath(full_path, artifacts_certs)
                zipf.write(full_path, relative_path)

    # Remove the temporary folder after zipping
    shutil.rmtree(artifacts_certs)
    
if __name__ == "__main__":
    extract_correct_certs("r3")
    extract_correct_certs("r4")
