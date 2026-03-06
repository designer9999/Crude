<!--
  Guide tab — explains how croc works, all features, setup for both PCs, FAQ
-->
<script lang="ts">
  import Card from "$lib/ui/Card.svelte";
  import Icon from "$lib/ui/Icon.svelte";
</script>

<div class="flex flex-col gap-4">

  <!-- What is croc -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="info" size={20} /></span>
      What is croc?
    </div>
    <p class="text-sm text-on-surface-variant leading-relaxed">
      <strong class="text-on-surface">croc</strong> is an open-source tool for encrypted peer-to-peer file transfer.
      Files are sent directly between two computers using a shared code phrase — no cloud storage, no upload limits, no accounts.
      Everything is end-to-end encrypted using PAKE (Password-Authenticated Key Exchange).
    </p>
  </Card>

  <!-- How it works -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="sync_alt" size={20} /></span>
      How it works
    </div>
    <div class="flex flex-col gap-3 text-sm text-on-surface-variant leading-relaxed">
      <div class="flex gap-3">
        <span class="shrink-0 w-6 h-6 rounded-full bg-primary text-on-primary flex items-center justify-center text-xs font-bold">1</span>
        <div><strong class="text-on-surface">Sender</strong> picks files or folders, then clicks <strong class="text-on-surface">Send</strong>. A unique transfer code appears (e.g. <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">1234-banana-apple-cherry</code>).</div>
      </div>
      <div class="flex gap-3">
        <span class="shrink-0 w-6 h-6 rounded-full bg-primary text-on-primary flex items-center justify-center text-xs font-bold">2</span>
        <div><strong class="text-on-surface">Sender</strong> shares that code with the receiver (chat, email, phone — any channel).</div>
      </div>
      <div class="flex gap-3">
        <span class="shrink-0 w-6 h-6 rounded-full bg-primary text-on-primary flex items-center justify-center text-xs font-bold">3</span>
        <div><strong class="text-on-surface">Receiver</strong> enters the code in the <strong class="text-on-surface">Receive</strong> tab and clicks <strong class="text-on-surface">Receive</strong>. The file downloads directly.</div>
      </div>
    </div>
  </Card>

  <!-- Features -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="stars" size={20} /></span>
      Features
    </div>
    <div class="grid grid-cols-2 gap-x-4 gap-y-2.5 text-sm">
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="lock" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">End-to-end encrypted</div>
          <div class="text-xs text-outline">PAKE + elliptic curves</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="speed" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">No size limit</div>
          <div class="text-xs text-outline">Stream any file size</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="restart_alt" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">Resumable</div>
          <div class="text-xs text-outline">Continue interrupted transfers</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="lan" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">Cross-network</div>
          <div class="text-xs text-outline">Works across NAT/firewall</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="folder_zip" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">Folders & multi-file</div>
          <div class="text-xs text-outline">Send anything, any amount</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="text_fields" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">Send text</div>
          <div class="text-xs text-outline">URLs, commands, snippets</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="compress" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">Auto-compression</div>
          <div class="text-xs text-outline">Built-in for faster transfer</div>
        </div>
      </div>
      <div class="flex items-start gap-2">
        <span class="text-primary shrink-0 mt-0.5"><Icon name="devices" size={16} /></span>
        <div>
          <div class="text-on-surface font-medium">Cross-platform</div>
          <div class="text-xs text-outline">Windows, Mac, Linux</div>
        </div>
      </div>
    </div>
  </Card>

  <!-- Options explained -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="tune" size={20} /></span>
      Send options explained
    </div>
    <div class="text-sm text-on-surface-variant leading-relaxed flex flex-col gap-3">
      <div>
        <p class="text-on-surface font-medium">Custom code phrase (reusable password)</p>
        <p>Set your own code instead of the random one. Must be 6+ characters. This code is <strong class="text-on-surface">saved across sessions</strong> — set it once on both PCs and you never need to type a new code again. Both sender and receiver must use the same code. Click "regenerate" only if you want a fresh random code.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Encryption curve</p>
        <p><strong class="text-on-surface">P-256</strong> (default) is fast and secure. <strong class="text-on-surface">P-521</strong> is strongest. <strong class="text-on-surface">Ed25519</strong> and <strong class="text-on-surface">SIEC</strong> are alternatives.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Hash algorithm</p>
        <p><strong class="text-on-surface">xxhash</strong> (default) is fastest. <strong class="text-on-surface">imohash</strong> is better for very large files. <strong class="text-on-surface">MD5</strong> for compatibility.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Zip folders</p>
        <p>Compresses a folder into a zip before sending. Receiver gets a .zip file. Useful for many small files.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Respect .gitignore</p>
        <p>When sending a project folder, automatically skips files listed in .gitignore (node_modules, .venv, etc.).</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Exclude patterns</p>
        <p>Comma-separated list of strings. Any file or folder whose path contains these strings will be skipped.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Disable compression</p>
        <p>Skip built-in compression. Faster for already-compressed files (videos, zips, images). Both sender and receiver must match this setting.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Local network only</p>
        <p>Transfer directly over LAN without using any relay. Much faster, but both PCs must be on the same network.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Speed limit</p>
        <p>Throttle upload speed. Use <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">500k</code> for 500 KB/s or <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">2M</code> for 2 MB/s. Useful on shared connections.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Quick access folder</p>
        <p>Set a default folder to browse files from. Saved across sessions — great if you always send files from the same place.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Save folder (receive)</p>
        <p>Choose where received files go. Saved across sessions — every time someone sends you a file, it goes to this folder automatically.</p>
      </div>
    </div>
  </Card>

  <!-- Persistent setup (reusable password) -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="link" size={20} /></span>
      One-time setup between two PCs
    </div>
    <div class="text-sm text-on-surface-variant leading-relaxed flex flex-col gap-3">
      <p>You can set up a <strong class="text-on-surface">permanent connection</strong> between two PCs so you never need to type a code again:</p>
      <ol class="list-decimal pl-5 flex flex-col gap-2">
        <li><strong class="text-on-surface">Sender:</strong> Go to Send &rarr; Options &rarr; set a <strong class="text-on-surface">Custom code phrase</strong> (e.g. <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">office-transfer-2024</code>)</li>
        <li><strong class="text-on-surface">Receiver:</strong> Just remember that code — type it every time in the Receive tab</li>
        <li>The sender's code is <strong class="text-on-surface">saved automatically</strong> — no need to set it again</li>
      </ol>
      <p class="text-on-surface-variant">Both PCs can also set a <strong class="text-on-surface">Quick access folder</strong> (sender) and <strong class="text-on-surface">Save folder</strong> (receiver) so files always go to the right place.</p>
    </div>
  </Card>

  <!-- Setup for your PC -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="computer" size={20} /></span>
      Setup — your PC
    </div>
    <div class="text-sm text-on-surface-variant leading-relaxed flex flex-col gap-2">
      <p>If you're reading this, your PC is already set up. Just use the <strong class="text-on-surface">Send</strong> or <strong class="text-on-surface">Receive</strong> tabs.</p>
      <p>croc, Python, and this app are all ready to go.</p>
    </div>
  </Card>

  <!-- Setup for colleague -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="group" size={20} /></span>
      Setup — colleague's PC
    </div>
    <div class="text-sm text-on-surface-variant leading-relaxed flex flex-col gap-2">
      <p><strong class="text-on-surface">Option A — Full app (recommended):</strong></p>
      <ol class="list-decimal pl-5 flex flex-col gap-1.5">
        <li>Copy the entire <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">Crude</code> folder to their PC</li>
        <li>Install croc: open terminal and run<br>
          <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">winget install schollz.croc</code></li>
        <li>Restart their terminal / PC</li>
        <li>Install Python 3.13 + pywebview:<br>
          <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">pip install pywebview</code></li>
        <li>Double-click <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">Croc-Transfer.bat</code></li>
      </ol>

      <div class="my-1 border-t border-outline-variant"></div>

      <p><strong class="text-on-surface">Option B — Command line only:</strong></p>
      <ol class="list-decimal pl-5 flex flex-col gap-1.5">
        <li>Install croc: <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">winget install schollz.croc</code></li>
        <li>Restart terminal</li>
        <li>To receive: <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">croc &lt;code&gt;</code></li>
        <li>To send: <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">croc send &lt;file&gt;</code></li>
      </ol>

      <div class="my-1 border-t border-outline-variant"></div>

      <p><strong class="text-on-surface">Other platforms:</strong></p>
      <div class="flex flex-col gap-1 pl-1">
        <div class="flex gap-2 items-baseline">
          <span class="text-on-surface font-medium text-xs w-14 shrink-0">macOS:</span>
          <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">brew install croc</code>
        </div>
        <div class="flex gap-2 items-baseline">
          <span class="text-on-surface font-medium text-xs w-14 shrink-0">Linux:</span>
          <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">curl https://getcroc.schollz.com | bash</code>
        </div>
      </div>
    </div>
  </Card>

  <!-- FAQ -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="help" size={20} /></span>
      FAQ
    </div>
    <div class="text-sm text-on-surface-variant leading-relaxed flex flex-col gap-3">
      <div>
        <p class="text-on-surface font-medium">Does it work across different networks?</p>
        <p>Yes. croc uses a relay server to connect through NATs and firewalls. Both PCs just need internet access — they don't need to be on the same network. On the same LAN, enable "Local network only" for fastest speeds.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Is it secure?</p>
        <p>Yes. Files are end-to-end encrypted using PAKE — the relay server never sees your data, only encrypted bytes. You can choose stronger encryption curves (P-521) in Options for extra security.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">File size limit?</p>
        <p>None. croc streams data in chunks and supports files of any size. Interrupted transfers can be resumed automatically.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Can I send folders?</p>
        <p>Yes. Folders are automatically compressed, transferred, and decompressed on the other end. Use the "Zip folders" option to send as a .zip instead.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Where do received files go?</p>
        <p>By default, files go to the folder where the app was launched. Use the "Choose folder" option in the Receive tab to pick a different location.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Can I send text instead of files?</p>
        <p>Yes! Switch to the "Text" mode in the Send tab. You can send URLs, commands, code snippets — any text content.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">What if the transfer fails?</p>
        <p>croc supports resumable transfers. Just run the same code again and it will pick up where it left off. Check the log panel for error details.</p>
      </div>
      <div>
        <p class="text-on-surface font-medium">Can I use a custom relay server?</p>
        <p>Yes. Advanced users can run their own relay with <code class="font-mono text-primary bg-surface-container-highest px-1.5 py-0.5 rounded text-xs">croc relay</code> and set it in Options. The default relay is maintained by the croc project.</p>
      </div>
    </div>
  </Card>

  <!-- CLI reference -->
  <Card>
    <div class="flex items-center gap-2 text-base font-medium text-on-surface mb-3">
      <span class="text-primary"><Icon name="terminal" size={20} /></span>
      CLI quick reference
    </div>
    <div class="text-sm text-on-surface-variant leading-relaxed flex flex-col gap-2">
      <div class="flex flex-col gap-1.5 font-mono text-xs">
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc send file.txt</code>
          <span class="text-outline self-center">Send a file</span>
        </div>
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc send *.jpg</code>
          <span class="text-outline self-center">Send multiple files</span>
        </div>
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc send folder/</code>
          <span class="text-outline self-center">Send a folder</span>
        </div>
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc send -c mycode file</code>
          <span class="text-outline self-center">Custom code</span>
        </div>
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc send --text "hello"</code>
          <span class="text-outline self-center">Send text</span>
        </div>
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc &lt;code&gt;</code>
          <span class="text-outline self-center">Receive</span>
        </div>
        <div class="flex gap-2">
          <code class="text-primary bg-surface-container-highest px-2 py-1 rounded shrink-0">croc --out ~/Downloads &lt;code&gt;</code>
          <span class="text-outline self-center">Receive to folder</span>
        </div>
      </div>
    </div>
  </Card>

</div>
