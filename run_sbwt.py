from util import *
import os


def index_path(fasta_file):
    prefix = f"{OUT_FOLDER}/{get_basename(fasta_file)}"
    return prefix + ".sbwt"


def build(fasta_file, **params):
    k = params["k"]
    os.makedirs(OUT_FOLDER, exist_ok=True)
    prefix = f"{OUT_FOLDER}/{get_basename(fasta_file)}"
    sbwt_file = prefix + ".sbwt"
    return measure_time(
        f"./SBWT/build/bin/sbwt build -i {fasta_file} -k {k} -o {sbwt_file} -t 1",
        **params
    )


def query(indexed_file, query_file, **params):
    prefix = f"{OUT_FOLDER}/{get_basename(indexed_file)}"
    sbwt_file = prefix + ".sbwt"
    return measure_time(
        f"./SBWT/build/bin/sbwt search -i {sbwt_file} -q {query_file} -o /dev/null",
        **params
    )
