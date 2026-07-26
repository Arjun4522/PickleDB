import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

interface BadgeProps {
  children: ReactNode
  variant?: 'emerald' | 'blue' | 'purple' | 'default'
  className?: string
}

export function Badge({ children, variant = 'default', className }: BadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium border',
        {
          'bg-emerald-500/10 text-emerald-400 border-emerald-500/20': variant === 'emerald',
          'bg-blue-500/10 text-blue-400 border-blue-500/20': variant === 'blue',
          'bg-purple-500/10 text-purple-400 border-purple-500/20': variant === 'purple',
          'bg-pickle-700/50 text-pickle-300 border-pickle-600/50': variant === 'default',
        },
        className,
      )}
    >
      {children}
    </span>
  )
}
