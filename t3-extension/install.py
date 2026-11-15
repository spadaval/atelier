#!/usr/bin/env python3
"""Install T3 Custom without changing the official T3 application."""
import os
import json
from pathlib import Path
import plistlib
import shlex
import shutil

source = Path(__file__).resolve().parent
support = Path.home() / 'Library/Application Support/T3 Custom'
app = Path.home() / 'Applications/T3 Custom.app'
official = Path('/Applications/T3 Code (Nightly).app')
with (official / 'Contents/Info.plist').open('rb') as file:
    official_info = plistlib.load(file)
runtime = official / 'Contents/MacOS' / official_info['CFBundleExecutable']
if not runtime.is_file():
    raise SystemExit(f'T3 executable does not exist: {runtime}')
support.mkdir(parents=True, exist_ok=True, mode=0o700)
for name in ['t3-custom.mjs', 'host-adapter.js', 'README.md']:
    shutil.copy2(source / name, support / name)
for name in ['custom.js', 'custom.css']:
    target = support / name
    if not target.exists():
        shutil.copy2(source / name, target)
config_path = support / 'config.json'
source_config = source / 'config.json'
if not source_config.exists():
    source_config = source / 'config.example.json'
if not config_path.exists():
    shutil.copy2(source_config, config_path)
config = json.loads(config_path.read_text())
if 'desk' not in config:
    config['desk'] = json.loads(source_config.read_text())['desk']
    config_path.write_text(json.dumps(config, indent=2) + '\n')

launcher = '#!/bin/sh\nexec /usr/bin/env ELECTRON_RUN_AS_NODE=1 ' + shlex.quote(str(runtime)) + ' ' + shlex.quote(str(support / 't3-custom.mjs')) + ' "$@"\n'
cli = support / 't3-custom'
cli.write_text(launcher)
cli.chmod(0o700)
contents = app / 'Contents'
(contents / 'MacOS').mkdir(parents=True, exist_ok=True)
(contents / 'Resources').mkdir(exist_ok=True)
(contents / 'MacOS/T3 Custom').write_text(launcher)
(contents / 'MacOS/T3 Custom').chmod(0o755)
shutil.copy2(official / 'Contents/Resources/icon.icns', contents / 'Resources/icon.icns')
with (contents / 'Info.plist').open('wb') as file:
    plistlib.dump({
        'CFBundleIdentifier': 'local.supadava.t3-custom',
        'CFBundleName': 'T3 Custom',
        'CFBundleDisplayName': 'T3 Custom',
        'CFBundleExecutable': 'T3 Custom',
        'CFBundlePackageType': 'APPL',
        'CFBundleShortVersionString': '1.0.0',
        'CFBundleVersion': '1',
        'CFBundleIconFile': 'icon.icns',
        'LSUIElement': True,
        'NSHighResolutionCapable': True,
    }, file)
print(f'Launcher: {app}')
print(f'Customizations and logs: {support}')
print(f'Control: {cli} status')
