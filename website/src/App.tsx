import { Navbar } from './components/Navbar'
import { Hero } from './components/Hero'
import { StatsBar } from './components/StatsBar'
import { Features } from './components/Features'
import { Architecture } from './components/Architecture'
import { Security } from './components/Security'
import { Comparison } from './components/Comparison'
import { Performance } from './components/Performance'
import { CodeExample } from './components/CodeExample'
import { Testimonials } from './components/Testimonials'
import { GitHubCta } from './components/GitHubCta'
import { Footer } from './components/Footer'

export default function App() {
  return (
    <div className="relative min-h-screen bg-pickle-950">
      <Navbar />
      <main>
        <Hero />
        <StatsBar />
        <Features />
        <Architecture />
        <Security />
        <Comparison />
        <Performance />
        <CodeExample />
        <Testimonials />
        <GitHubCta />
      </main>
      <Footer />
    </div>
  )
}
