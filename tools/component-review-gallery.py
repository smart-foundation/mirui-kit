#!/usr/bin/env python3
"""Package recovered, provenance-checked native PNGs into a portable review page."""
import argparse
import base64
import hashlib
from html import escape
import json
from pathlib import Path
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('directory',type=Path)
args=parser.parse_args()
evidence=json.loads((args.directory/'evidence.json').read_text())
cards=[]
for record in evidence['images']:
    data=(args.directory/record['file']).read_bytes()
    if hashlib.sha256(data).hexdigest()!=record['sha256']:raise SystemExit('PNG provenance mismatch')
    source='data:image/png;base64,'+base64.b64encode(data).decode('ascii')
    label=escape(record['file'].removesuffix('.png').replace('--',' · '))
    cards.append(f'''<article><h2>{label}</h2><div class="image"><img src="{source}" alt="Exact native {label} capture"></div><button onclick="this.previousElementSibling.classList.toggle('actual')">Toggle actual pixel size</button><details><summary>Native evidence provenance</summary><p>{record['viewport'][0]} × {record['viewport'][1]} · check run {record['check_run']}</p><code>{record['sha256']}</code></details></article>''')
page='''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>Mirui native component review</title><style>
*{box-sizing:border-box}body{margin:0;background:#f4f5f7;color:#202127;font:15px/1.5 system-ui,sans-serif}header,main{max-width:1300px;margin:auto;padding:24px}header{padding-bottom:0}h1{font-size:30px;letter-spacing:-.7px;margin:8px 0}h2{font-size:15px;margin:0 0 12px}p{max-width:90ch}a{color:#075fc5}.status{display:inline-block;color:#6f4300;background:#fff0ca;border:1px solid #dfba67;padding:4px 10px;border-radius:6px;font-weight:600}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,390px),1fr));gap:20px}article{padding:16px;background:white;border:1px solid #d5d7de;border-radius:10px;min-width:0}.image{height:420px;overflow:auto;border:1px solid #d9dce3;border-radius:5px;background:white}.image img{display:block;width:100%;height:auto}.image.actual img{width:auto;max-width:none}button{cursor:pointer;color:#202127;background:#f0f1f5;border:1px solid #d0d3dc;border-radius:5px;padding:7px 12px;margin:12px 0}button:focus-visible,summary:focus-visible{outline:2px solid #0968d4;outline-offset:3px}summary{cursor:pointer}code{font-size:11px;overflow-wrap:anywhere}footer{padding:24px;text-align:center;color:#626671}
</style><header><span class="status">Evidence — not approved baselines</span><h1>Mirui native component review</h1><p>Real Mirui framebuffer captures, not generated mockups or an alternate renderer. The full CI matrix produced 288 state/theme/text-scale/width captures across 13 isolated families. This page contains the 15 representative PNGs exported through Checks; it does not claim exhaustive state review or complete kit parity.</p><p>Source: <a href="RUN_URL">CI RUN_ID</a> · commit <code>COMMIT</code>. Images are embedded unchanged and checked against their SHA-256 provenance. Scroll inside a capture or toggle actual pixel size to inspect text and edges.</p><p><strong>First inspection found five issues:</strong> numeric-step alignment, centred listbox rows, square switch outline, stretched badge, and heavy slider paint. See <a href="https://github.com/smart-foundation/mirui-kit/blob/arena/b695798f-mirui-kit/docs/ux-audit/COMPONENT_VISUAL_REVIEW.md">the review record</a> for fixes, verification status and open work. Successful rendering is not visual or UX acceptance.</p></header><main class="grid">CARDS</main><footer>No captures are automatically approved by this page.</footer></html>'''
page=page.replace('RUN_URL',escape(evidence['url'])).replace('RUN_ID',str(evidence['run'])).replace('COMMIT',escape(evidence['source_commit'])).replace('CARDS',''.join(cards))
output=args.directory/'review.html';output.write_text(page)
print(output)
