#!/usr/bin/env python3
"""Recover exact native PNG notices from a successful component capture run.

Requires the authenticated gh CLI. No artifact redirect hosts or image models.
"""
import argparse
import base64
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('run',type=int)
parser.add_argument('--out',type=Path,default=Path('artifacts/studio/component-review'))
args=parser.parse_args()
def gh(*command):return json.loads(subprocess.check_output(['gh',*command]))
run=gh('run','view',str(args.run),'--json','jobs,headSha,conclusion,url')
if run['conclusion']!='success':raise SystemExit('Only successful native runs are accepted as complete capture evidence')
images={}
for job in run['jobs']:
    if not job['name'].startswith('component-review'):continue
    if job['conclusion']!='success':raise SystemExit('Incomplete component capture shard')
    annotations=gh('api',f'repos/smart-foundation/mirui-kit/check-runs/{job["databaseId"]}/annotations?per_page=100')
    for annotation in annotations:
        match=re.fullmatch(r'Native PNG ([\w.-]+) (\d+)/(\d+)',annotation.get('title',''))
        if not match:continue
        name,number,total=match.groups()
        image=images.setdefault(name,{'total':int(total),'chunks':{},'job':job['databaseId']})
        if int(number) in image['chunks']:raise SystemExit('Duplicate evidence chunk')
        image['chunks'][int(number)]=annotation['message']
if not images:raise SystemExit('No native component PNG evidence in this run')
args.out.mkdir(parents=True,exist_ok=True)
spec=importlib.util.spec_from_file_location('native_png',Path(__file__).with_name('compare-native-captures.py'))
native_png=importlib.util.module_from_spec(spec);spec.loader.exec_module(native_png)
records=[]
for name,image in sorted(images.items()):
    if set(image['chunks'])!=set(range(1,image['total']+1)):raise SystemExit('Incomplete PNG: '+name)
    data=base64.b64decode(''.join(image['chunks'][i] for i in range(1,image['total']+1)),validate=True)
    path=args.out/name;path.write_bytes(data)
    width,height,_=native_png.rgba(path)
    records.append({'file':name,'sha256':hashlib.sha256(data).hexdigest(),'viewport':[width,height],'check_run':image['job']})
manifest={'schema':1,'source_commit':run['headSha'],'run':args.run,'url':run['url'],'coverage':'representative resting specimens only; complete state matrix is in CI artifacts','visually_approved':False,'images':records}
(args.out/'evidence.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(f"Recovered {len(records)} exact native PNGs from {run['headSha']}; visual approval still required")
