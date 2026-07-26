import { useState } from 'react'
import { motion } from 'framer-motion'
import { Copy, Check } from 'lucide-react'

interface Token {
  type: 'keyword' | 'string' | 'comment' | 'function' | 'type' | 'number' | 'macro' | 'plain'
  value: string
}

const syntaxHighlight: Record<string, Token[][]> = {
  rust: [
    [{ type: 'keyword', value: 'use' }, { type: 'plain', value: ' pickledb::' }, { type: 'type', value: 'Database' }, { type: 'plain', value: ';' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'keyword', value: 'let' }, { type: 'plain', value: ' mut db = ' }, { type: 'type', value: 'Database' }, { type: 'plain', value: '::' }, { type: 'function', value: 'open' }, { type: 'plain', value: '(' }, { type: 'string', value: '"app.pdb"' }, { type: 'plain', value: ')?;' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'comment', value: '// Insert encrypted record' }],
    [{ type: 'keyword', value: 'let' }, { type: 'plain', value: ' record = ' }, { type: 'macro', value: 'serde_json::json!' }, { type: 'plain', value: '!({' }],
    [{ type: 'plain', value: '    ' }, { type: 'string', value: '"email"' }, { type: 'plain', value: ': ' }, { type: 'string', value: '"alice@example.com"' }, { type: 'plain', value: ',' }],
    [{ type: 'plain', value: '    ' }, { type: 'string', value: '"name"' }, { type: 'plain', value: ': ' }, { type: 'string', value: '"Alice"' }, { type: 'plain', value: ',' }],
    [{ type: 'plain', value: '    ' }, { type: 'string', value: '"role"' }, { type: 'plain', value: ': ' }, { type: 'string', value: '"admin"' }],
    [{ type: 'plain', value: '});' }],
    [{ type: 'plain', value: 'db.' }, { type: 'function', value: 'insert' }, { type: 'plain', value: '(' }, { type: 'string', value: '"user_001"' }, { type: 'plain', value: ', &record)?;' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'comment', value: '// Search without decrypting' }],
    [{ type: 'keyword', value: 'let' }, { type: 'plain', value: ' results = db' }],
    [{ type: 'plain', value: '    .' }, { type: 'function', value: 'search' }, { type: 'plain', value: '(' }, { type: 'string', value: '"email"' }, { type: 'plain', value: ')' }],
    [{ type: 'plain', value: '    .' }, { type: 'function', value: 'eq' }, { type: 'plain', value: '(' }, { type: 'string', value: '"alice@example.com"' }, { type: 'plain', value: ')' }],
    [{ type: 'plain', value: '    .' }, { type: 'function', value: 'execute' }, { type: 'plain', value: '()?;' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'keyword', value: 'for' }, { type: 'plain', value: ' record ' }, { type: 'keyword', value: 'in' }, { type: 'plain', value: ' results {' }],
    [{ type: 'plain', value: '    ' }, { type: 'macro', value: 'println!' }, { type: 'plain', value: '(' }, { type: 'string', value: '"Found: {:?}"' }, { type: 'plain', value: ', record);' }],
    [{ type: 'plain', value: '}' }],
  ],
  python: [
    [{ type: 'keyword', value: 'from' }, { type: 'plain', value: ' pickledb ' }, { type: 'keyword', value: 'import' }, { type: 'plain', value: ' ' }, { type: 'type', value: 'PickleDB' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'plain', value: 'db = ' }, { type: 'type', value: 'PickleDB' }, { type: 'plain', value: '.' }, { type: 'function', value: 'open' }, { type: 'plain', value: '(' }, { type: 'string', value: '"app.pdb"' }, { type: 'plain', value: ')' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'comment', value: '# Insert encrypted record' }],
    [{ type: 'plain', value: 'record = {' }],
    [{ type: 'plain', value: '    ' }, { type: 'string', value: '"email"' }, { type: 'plain', value: ': ' }, { type: 'string', value: '"alice@example.com"' }, { type: 'plain', value: ',' }],
    [{ type: 'plain', value: '    ' }, { type: 'string', value: '"name"' }, { type: 'plain', value: ': ' }, { type: 'string', value: '"Alice"' }, { type: 'plain', value: ',' }],
    [{ type: 'plain', value: '    ' }, { type: 'string', value: '"role"' }, { type: 'plain', value: ': ' }, { type: 'string', value: '"admin"' }],
    [{ type: 'plain', value: '}' }],
    [{ type: 'plain', value: 'db.' }, { type: 'function', value: 'insert' }, { type: 'plain', value: '(' }, { type: 'string', value: '"user_001"' }, { type: 'plain', value: ', record)' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'comment', value: '# Search without decrypting' }],
    [{ type: 'plain', value: 'results = db.' }, { type: 'function', value: 'search' }, { type: 'plain', value: '(' }, { type: 'string', value: '"email"' }, { type: 'plain', value: ').' }, { type: 'function', value: 'eq' }, { type: 'plain', value: '(' }, { type: 'string', value: '"alice@example.com"' }, { type: 'plain', value: ').' }, { type: 'function', value: 'execute' }, { type: 'plain', value: '()' }],
    [{ type: 'keyword', value: 'for' }, { type: 'plain', value: ' record ' }, { type: 'keyword', value: 'in' }, { type: 'plain', value: ' results:' }],
    [{ type: 'plain', value: '    ' }, { type: 'function', value: 'print' }, { type: 'plain', value: '(' }, { type: 'keyword', value: 'f' }, { type: 'string', value: '"Found: {record}"' }, { type: 'plain', value: ')' }],
  ],
  typescript: [
    [{ type: 'keyword', value: 'import' }, { type: 'plain', value: ' { ' }, { type: 'type', value: 'PickleDB' }, { type: 'plain', value: ' } ' }, { type: 'keyword', value: 'from' }, { type: 'plain', value: ' ' }, { type: 'string', value: "'pickledb'" }],
    [{ type: 'plain', value: '' }],
    [{ type: 'keyword', value: 'const' }, { type: 'plain', value: ' db = ' }, { type: 'keyword', value: 'await' }, { type: 'plain', value: ' ' }, { type: 'type', value: 'PickleDB' }, { type: 'plain', value: '.' }, { type: 'function', value: 'open' }, { type: 'plain', value: '(' }, { type: 'string', value: "'app.pdb'" }, { type: 'plain', value: ')' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'comment', value: '// Insert encrypted record' }],
    [{ type: 'keyword', value: 'await' }, { type: 'plain', value: ' db.' }, { type: 'function', value: 'insert' }, { type: 'plain', value: '(' }, { type: 'string', value: "'user_001'" }, { type: 'plain', value: ', {' }],
    [{ type: 'plain', value: '  ' }, { type: 'plain', value: 'email' }, { type: 'plain', value: ': ' }, { type: 'string', value: "'alice@example.com'" }, { type: 'plain', value: ',' }],
    [{ type: 'plain', value: '  ' }, { type: 'plain', value: 'name' }, { type: 'plain', value: ': ' }, { type: 'string', value: "'Alice'" }, { type: 'plain', value: ',' }],
    [{ type: 'plain', value: '  ' }, { type: 'plain', value: 'role' }, { type: 'plain', value: ': ' }, { type: 'string', value: "'admin'" }],
    [{ type: 'plain', value: '})' }],
    [{ type: 'plain', value: '' }],
    [{ type: 'comment', value: '// Search without decrypting' }],
    [{ type: 'keyword', value: 'const' }, { type: 'plain', value: ' results = ' }, { type: 'keyword', value: 'await' }, { type: 'plain', value: ' db' }],
    [{ type: 'plain', value: '  .' }, { type: 'function', value: 'search' }, { type: 'plain', value: '(' }, { type: 'string', value: "'email'" }, { type: 'plain', value: ')' }],
    [{ type: 'plain', value: '  .' }, { type: 'function', value: 'eq' }, { type: 'plain', value: '(' }, { type: 'string', value: "'alice@example.com'" }, { type: 'plain', value: ')' }],
    [{ type: 'plain', value: '  .' }, { type: 'function', value: 'execute' }, { type: 'plain', value: '()' }],
  ],
}

const colorClass: Record<Token['type'], string> = {
  keyword: 'text-violet-400',
  string: 'text-emerald-400',
  comment: 'text-pickle-500',
  function: 'text-cyan-400',
  type: 'text-cyan-300',
  number: 'text-amber-400',
  macro: 'text-amber-300',
  plain: 'text-pickle-200',
}

const rawCode: Record<string, string> = {
  rust: `use pickledb::Database;

let mut db = Database::open("app.pdb")?;

// Insert encrypted record
let record = serde_json::json!({
    "email": "alice@example.com",
    "name": "Alice",
    "role": "admin"
});
db.insert("user_001", &record)?;

// Search without decrypting
let results = db
    .search("email")
    .eq("alice@example.com")
    .execute()?;

for record in results {
    println!("Found: {:?}", record);
}`,
  python: `from pickledb import PickleDB

db = PickleDB.open("app.pdb")

# Insert encrypted record
record = {
    "email": "alice@example.com",
    "name": "Alice",
    "role": "admin"
}
db.insert("user_001", record)

# Search without decrypting
results = db.search("email").eq("alice@example.com").execute()
for record in results:
    print(f"Found: {record}")`,
  typescript: `import { PickleDB } from 'pickledb'

const db = await PickleDB.open('app.pdb')

// Insert encrypted record
await db.insert('user_001', {
  email: 'alice@example.com',
  name: 'Alice',
  role: 'admin'
})

// Search without decrypting
const results = await db
  .search('email')
  .eq('alice@example.com')
  .execute()`,
}

const languages = [
  { id: 'rust' as const, label: 'Rust' },
  { id: 'python' as const, label: 'Python' },
  { id: 'typescript' as const, label: 'TypeScript' },
]

export function CodeExample() {
  const [lang, setLang] = useState<'rust' | 'python' | 'typescript'>('rust')
  const [copied, setCopied] = useState(false)

  const copy = async () => {
    await navigator.clipboard.writeText(rawCode[lang])
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <section className="relative py-16 lg:py-24">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="max-w-3xl mx-auto">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="mb-10"
          >
            <span className="text-xs uppercase tracking-widest text-emerald-400/70 font-medium">Code</span>
            <h2 className="mt-4 font-heading text-3xl sm:text-4xl lg:text-5xl font-bold text-white tracking-tight">
              Simple <span className="text-emerald-400">API</span>
            </h2>
          </motion.div>

          <div className="rounded-2xl border border-border bg-surface overflow-hidden">
            <div className="flex items-center border-b border-border">
              {languages.map((l) => (
                <button
                  key={l.id}
                  onClick={() => setLang(l.id)}
                  className={`px-4 py-2.5 text-xs font-mono font-medium transition-colors ${
                    lang === l.id
                      ? 'text-emerald-400 bg-emerald-500/[0.04]'
                      : 'text-pickle-400 hover:text-pickle-200'
                  }`}
                >
                  {l.label}
                </button>
              ))}
              <button
                onClick={copy}
                className="ml-auto px-4 py-2.5 text-pickle-400 hover:text-white transition-colors"
              >
                {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              </button>
            </div>
            <div className="p-5 overflow-x-auto">
              <pre className="text-sm font-mono leading-relaxed whitespace-pre">
                <code>
                  {syntaxHighlight[lang].map((tokens, lineIdx) => (
                    <div key={lineIdx} className="flex">
                      <span className="select-none text-pickle-600 w-8 text-right mr-5 text-xs leading-relaxed">{lineIdx + 1}</span>
                      <span>
                        {tokens.map((token, tIdx) => (
                          <span key={tIdx} className={colorClass[token.type]}>{token.value}</span>
                        ))}
                      </span>
                    </div>
                  ))}
                </code>
              </pre>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
