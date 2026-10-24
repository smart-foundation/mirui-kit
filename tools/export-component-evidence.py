#!/usr/bin/env python3
"""Transport representative exact native PNG bytes through the Checks API.

Artifacts retain every state. The eight review shards additionally export every
family's resting specimen for restricted reviewers. No images are synthesized.
"""
import base64
import json
from pathlib import Path
import sys
root=Path(sys.argv[1]);batch=int(sys.argv[2])
if not 1<=batch<=4:raise SystemExit('batch must be 1..4')
index=json.loads((root/'index.json').read_text())
groups={('dark',100,640):['button','field'],('dark',100,960):['numeric-entry','checkbox'],('dark',150,640):['switch','slider'],('dark',150,960):['choice-group','listbox'],('light',100,640):['tree','table'],('light',100,960):['calendar','split'],('light',150,640):['feedback'],('light',150,960):['button','field']}
names=groups[index['theme'],index['scale'],index['viewport'][0]]
notices=[]
for name in names:
    matches=[case for case in index['captures'] if case['component']==name and case['state']=='rest']
    if len(matches)!=1:raise SystemExit('representative capture missing or ambiguous: '+name)
    filename=matches[0]['file'];data=(root/filename).read_bytes()
    if data[:8]!=b'\x89PNG\r\n\x1a\n':raise SystemExit('not native PNG')
    encoded=base64.b64encode(data).decode('ascii');chunks=[encoded[i:i+4000] for i in range(0,len(encoded),4000)]
    notices.extend((filename,i+1,len(chunks),chunk) for i,chunk in enumerate(chunks))
if len(notices)>40:raise SystemExit('representative bytes exceed Checks transport capacity; retain artifacts')
for filename,number,total,chunk in notices[(batch-1)*10:batch*10]:
    print(f'::notice title=Native PNG {filename} {number}/{total}::{chunk}')
