#!/usr/bin/env python3
"""Fiscalização auxiliar local; aceita somente D/V e controles próprios, nunca F."""
import argparse
import glob
import os
import signal
import subprocess
import sys
import time

LIMIT_RSS = 256 * 1024 * 1024
LIMIT_TIME = 30.0
INTERVAL = 0.002


def group_rss(pgid):
    total = 0
    count = 0
    for name in os.listdir('/proc'):
        if not name.isdecimal():
            continue
        try:
            with open('/proc/' + name + '/stat', encoding='ascii') as f:
                data = f.read()
            fields = data[data.rfind(')') + 2:].split()
            if int(fields[2]) != pgid:
                continue
            with open('/proc/' + name + '/status', encoding='ascii') as f:
                status = f.read().splitlines()
            rss = next((int(x.split()[1]) * 1024 for x in status if x.startswith('VmRSS:')), 0)
            total += rss
            count += 1
        except (OSError, ValueError, IndexError):
            continue
    return total, count


def supervise(args, seconds=LIMIT_TIME, rss_limit=LIMIT_RSS):
    started = time.monotonic()
    proc = subprocess.Popen(args, start_new_session=True)
    peak, seen, reason = 0, 0, None
    try:
        while True:
            current, count = group_rss(proc.pid)
            peak, seen = max(peak, current), max(seen, count)
            elapsed = time.monotonic() - started
            if elapsed >= seconds:
                reason = 'timeout'
            elif current >= rss_limit:
                reason = 'rss_sample_limit'
            if reason:
                try:
                    os.killpg(proc.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                break
            if proc.poll() is not None:
                break
            time.sleep(INTERVAL)
        code = proc.wait()
    finally:
        if proc.poll() is None:
            os.killpg(proc.pid, signal.SIGKILL)
            proc.wait()
    print(f'reason={reason or "exit"} code={code} monotonic_s={time.monotonic()-started:.6f} sampled_group_rss_bytes={peak} max_observed_processes={seen}', flush=True)
    return reason, code


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['D', 'V', 'timeout-control', 'rss-control'])
    mode = parser.parse_args().mode
    if mode == 'timeout-control':
        reason, _ = supervise([sys.executable, '-c', 'import time; time.sleep(3)'], seconds=0.15)
        return 0 if reason == 'timeout' else 1
    if mode == 'rss-control':
        reason, _ = supervise([sys.executable, '-c', 'import time; x=bytearray(16*1024*1024); time.sleep(3)'], rss_limit=8*1024*1024)
        return 0 if reason == 'rss_sample_limit' else 1
    binaries = [p for p in glob.glob('target/debug/deps/v03_f1_p01_instruments-*') if os.path.isfile(p) and os.access(p, os.X_OK)]
    if len(binaries) != 1:
        print('instrumento ausente ou ambíguo; interrompido', file=sys.stderr)
        return 2
    name = 'development' if mode == 'D' else 'validation'
    reason, code = supervise([binaries[0], '--exact', name, '--nocapture'])
    return 0 if reason is None and code == 0 else 1


if __name__ == '__main__':
    raise SystemExit(main())
