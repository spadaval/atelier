#!/usr/bin/env python3
"""Install or upgrade T3 Custom without changing the official T3 application."""
import json
import os
import shlex
import shutil
import subprocess
import sys
from pathlib import Path

source = Path(__file__).resolve().parent
windows = sys.platform == 'win32'
# --home installs a separate copy (for example a test instance) without a launcher.
make_launcher = '--home' not in sys.argv
support = Path(sys.argv[sys.argv.index('--home') + 1]) if '--home' in sys.argv else (
    Path(os.environ['APPDATA']) / 'T3 Custom' if windows else Path.home() / 'Library/Application Support/T3 Custom')

support.mkdir(parents=True, exist_ok=True, mode=0o700)
# Managed files are replaced on every install; a changed custom.js is kept beside it.
for name in ['t3-custom.mjs', 'host-adapter.js', 'custom.js', 'README.md']:
    target = support / name
    if name == 'custom.js' and target.exists() and target.read_bytes() != (source / name).read_bytes():
        shutil.copy2(target, support / 'custom.js.previous')
        print(f'Replaced a modified {target}; the old copy is custom.js.previous')
    shutil.copy2(source / name, target)
if not (support / 'custom.css').exists():
    shutil.copy2(source / 'custom.css', support / 'custom.css')

config_path = support / 'config.json'
source_config = source / 'config.json'
if not source_config.exists():
    source_config = source / 'config.example.json'
config = json.loads(config_path.read_text()) if config_path.exists() else {}
for key, value in json.loads(source_config.read_text()).items():
    config.setdefault(key, value)
if not Path(config['appPath']).exists():
    found = sorted((Path(os.environ['LOCALAPPDATA']) / 'Programs/t3code').glob('T3 Code*.exe')) if windows else \
        [Path('/Applications/T3 Code (Nightly).app')]
    found = [path for path in found if path.exists() and not path.name.startswith('Uninstall')]
    if not found:
        raise SystemExit(f'T3 Code was not found at {config["appPath"]}. Set appPath in {config_path}.')
    config['appPath'] = str(found[0])
config_path.write_text(json.dumps(config, indent=2) + '\n')

if windows:
    # T3's own executable runs the loader in Node mode, so no separate Node is needed.
    runtime = Path(config['appPath'])
    cli = support / 't3-custom.cmd'
    cli.write_text(f'@echo off\r\nset "ELECTRON_RUN_AS_NODE=1"\r\n"{runtime}" "%~dp0t3-custom.mjs" %*\r\n')
    launcher = Path(os.environ['APPDATA']) / 'Microsoft/Windows/Start Menu/Programs/T3 Custom.lnk'
    if make_launcher:
        # A minimized cmd sets the variable and exits at once; the loader itself has no window.
        arguments = f'/d /c "set ELECTRON_RUN_AS_NODE=1&& start "" "{runtime}" "{support / "t3-custom.mjs"}" launch"'
        script = ('$s = (New-Object -ComObject WScript.Shell).CreateShortcut($env:LNK); $s.TargetPath = $env:ComSpec; '
                  '$s.Arguments = $env:ARGS; $s.WorkingDirectory = $env:HOME_DIR; $s.IconLocation = $env:ICON; $s.WindowStyle = 7; $s.Save()')
        subprocess.run(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command', script], check=True, env={
            **os.environ, 'LNK': str(launcher), 'ARGS': arguments, 'HOME_DIR': str(support), 'ICON': f'{runtime},0'})
else:
    import plistlib
    official = Path(config['appPath'])
    with (official / 'Contents/Info.plist').open('rb') as file:
        runtime = official / 'Contents/MacOS' / plistlib.load(file)['CFBundleExecutable']
    command = '#!/bin/sh\nexec /usr/bin/env ELECTRON_RUN_AS_NODE=1 ' + shlex.quote(str(runtime)) + ' ' + shlex.quote(str(support / 't3-custom.mjs')) + ' "$@"\n'
    cli = support / 't3-custom'
    cli.write_text(command)
    cli.chmod(0o700)
    launcher = Path.home() / 'Applications/T3 Custom.app'
if not windows and make_launcher:
    contents = launcher / 'Contents'
    (contents / 'MacOS').mkdir(parents=True, exist_ok=True)
    (contents / 'Resources').mkdir(exist_ok=True)
    (contents / 'MacOS/T3 Custom').write_text(command)
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
print(f'Launcher: {launcher}' if make_launcher else 'Launcher: not created for a --home install')
print(f'Customizations and logs: {support}')
print(f'Control: {cli} status')
