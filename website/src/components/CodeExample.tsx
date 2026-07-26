import { useState } from 'react'
import { motion } from 'framer-motion'
import { Copy, Check } from 'lucide-react'

const code = {
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
  .execute()
`,
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
    await navigator.clipboard.writeText(code[lang])
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <section className="relative py-24 lg:py-32">
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
              <pre className="text-sm font-mono leading-relaxed text-pickle-200 whitespace-pre">
                <code>{code[lang]}</code>
              </pre>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
