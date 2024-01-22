from util import *
import os


def build(fasta_file, k):
    os.makedirs(OUT_FOLDER, exist_ok=True)
    prefix = f"{OUT_FOLDER}/{get_basename(fasta_file)}"
    sbwt_file = prefix + ".sbwt"
    return measure_time(
        f"./SBWT/build/bin/sbwt build -i {fasta_file} -k {k} -o {sbwt_file} -t 1"
    )


def clean(fasta_file):
    prefix = f"{OUT_FOLDER}/{get_basename(fasta_file)}"
    sbwt_file = prefix + ".sbwt"
    os.remove(sbwt_file)
