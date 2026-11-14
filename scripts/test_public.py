#!/usr/bin/env python3
"""Regressões públicas explícitas; não descobre/executa outros alvos Cargo."""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
TARGETS = (
    "cli", "engine", "interactions", "learning", "memory", "persistence",
    "personality", "f1_session", "mp05_synaptic", "mp06_paths",
    "mp07_investigation", "mp08_regulation", "mp09_pedagogy",
    "concept_goal_supervision", "composed_query", "tcr01_transfer",
    "composed_goal_supervision", "composed_cli", "f3_01_composed_two",
    "experiential_cycle", "development_runtime", "development_actions", "development_consequences", "development_predictive", "development_archive",
)
EXCLUDED = ("bounded_update_probe", "bounded_cost_probe_for_activation_and_communication")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--build", action="store_true")
    parser.add_argument("--target", action="append", choices=TARGETS)
    parser.add_argument("--target-dir", type=Path, default=Path("/tmp/neurons-pifi-target"))
    args = parser.parse_args()
    selected = args.target or list(TARGETS)
    if args.list:
        print(json.dumps({"targets": selected, "excluded": EXCLUDED}, indent=2))
        return 0
    target_dir = args.target_dir.expanduser().resolve()
    if target_dir == ROOT or ROOT in target_dir.parents:
        parser.error("target-dir deve permanecer fora do repositório")
    if shutil.disk_usage(ROOT).free < 3 * 1024**3:
        parser.error("reserva mínima de disco: 3 GiB")
    env = dict(os.environ, CARGO_TARGET_DIR=str(target_dir))
    # Workers só recebem entradas geradas pelos próprios testes-pai. Uma variável
    # herdada da sessão não deve transformar o teste normal em leitura de arquivo.
    for key in ("NEURONS_FUNCTIONAL_CHECKPOINT", "NEURONS_PIFI_RESTORE", "NEURONS_ACTION_RESTORE", "NEURONS_CONSEQUENCE_RESTORE", "NEURONS_PREDICTIVE_RESTORE", "NEURONS_ARCHIVE_RESTORE"):
        env.pop(key, None)
    if args.build:
        command = ["cargo", "build", "--offline", "--locked", "--lib", "--bin", "neurons"]
        print("COMMAND", json.dumps(command), flush=True)
        result = subprocess.run(command, cwd=ROOT, env=env, timeout=120)
        if result.returncode:
            return result.returncode
    command = ["cargo", "test", "--offline", "--locked", "--quiet"]
    for name in selected:
        command += ["--test", name]
    command += ["--"]
    for name in EXCLUDED:
        command += ["--skip", name]
    print("COMMAND", json.dumps(command), flush=True)
    result = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True, timeout=120)
    print(result.stdout, end="")
    print(result.stderr, end="", file=sys.stderr)
    counts = re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;.*? (\d+) filtered out", result.stdout)
    totals = [sum(int(row[i]) for row in counts) for i in range(4)]
    print("PUBLIC_RESULT", json.dumps(dict(zip(("passed", "failed", "ignored", "filtered"), totals))))
    if result.returncode == 0 and len(counts) != len(selected):
        print("Resultado incompleto: quantidade de alvos não corresponde aos relatórios", file=sys.stderr)
        return 1
    return result.returncode


if __name__ == "__main__":
    try:
        sys.exit(main())
    except subprocess.TimeoutExpired as error:
        print(f"Interrompido pelo timeout: {error.cmd}", file=sys.stderr)
        sys.exit(124)
