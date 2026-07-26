import type { HTMLAttributes, ReactNode } from 'react'
import { cn } from '@/lib/utils'

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode
  variant?: 'default' | 'glass' | 'elevated'
  glow?: 'green' | 'blue' | 'purple' | 'none'
}

export function Card({
  children,
  variant = 'default',
  glow = 'none',
  className,
  ...props
}: CardProps) {
  return (
    <div
      className={cn(
        'rounded-2xl transition-all duration-500',
        {
          'bg-surface border border-border': variant === 'default',
          'glass': variant === 'glass',
          'bg-surface-elevated border border-border-light shadow-lg': variant === 'elevated',
        },
        {
          'hover:glow-green': glow === 'green',
          'hover:glow-blue': glow === 'blue',
          'hover:glow-purple': glow === 'purple',
        },
        className,
      )}
      {...props}
    >
      {children}
    </div>
  )
}
