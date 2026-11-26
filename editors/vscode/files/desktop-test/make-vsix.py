import json
from pathlib import Path
import sys
from zipfile import ZipFile, ZIP_DEFLATED

manifest = {
    "name": "desktop-acceptance",
    "displayName": "OpenBSD Desktop Acceptance",
    "version": "1.0.0",
    "publisher": "openbsd-local",
    "engines": {"vscode": "^1.148.0"},
    "main": "./extension.js",
    "activationEvents": ["onStartupFinished"],
    "contributes": {
        "commands": [{"command": "openbsd.twoTerminals", "title": "OpenBSD: Test Two Terminals"}],
    },
}
with ZipFile(sys.argv[1], "w", ZIP_DEFLATED) as out:
    out.writestr("extension/package.json", json.dumps(manifest))
    out.write(Path(__file__).with_name("extension.js"), "extension/extension.js")
    out.writestr("[Content_Types].xml", """<?xml version="1.0"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="js" ContentType="application/javascript"/>
<Default Extension="json" ContentType="application/json"/>
<Default Extension="vsixmanifest" ContentType="text/xml"/></Types>""")
    out.writestr("extension.vsixmanifest", """<?xml version="1.0"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
<Metadata><Identity Language="en-US" Id="desktop-acceptance" Version="1.0.0" Publisher="openbsd-local"/>
<DisplayName>OpenBSD Desktop Acceptance</DisplayName><Description>Local desktop test</Description>
<Categories>Other</Categories><Properties>
<Property Id="Microsoft.VisualStudio.Code.Engine" Value="^1.148.0"/>
</Properties></Metadata><Installation><InstallationTarget Id="Microsoft.VisualStudio.Code"/></Installation>
<Dependencies/><Assets><Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json"
Addressable="true"/></Assets></PackageManifest>""")
