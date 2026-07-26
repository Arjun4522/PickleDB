import type { ButtonHTMLAttributes, ReactNode } from 'react'
import { cn } from '@/lib/utils'

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  icon?: ReactNode
  href?: string
}

export function Button({
  variant = 'primary',
  size = 'md',
  className,
  children,
  icon,
  href,
  ...props
}: ButtonProps) {
  const base = cn(
    'inline-flex items-center justify-center gap-2 font-medium rounded-xl transition-all duration-300 cursor-pointer',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-emerald-500/50',
    'disabled:opacity-50 disabled:pointer-events-none',
    {
      'bg-emerald-500 text-white hover:bg-emerald-400 hover:shadow-lg hover:shadow-emerald-500/25 active:scale-[0.98]':
        variant === 'primary',
      'glass text-pickle-200 hover:bg-white/10 hover:text-white active:scale-[0.98]':
        variant === 'secondary',
      'text-pickle-400 hover:text-white hover:bg-white/5 active:scale-[0.98]':
        variant === 'ghost',
    },
    {
      'px-3 py-1.5 text-sm': size === 'sm',
      'px-5 py-2.5 text-sm': size === 'md',
      'px-8 py-3.5 text-base': size === 'lg',
    },
    className,
  )

  if (href) {
    return (
      <a href={href} className={base}>
        {icon && <span className="w-4 h-4">{icon}</span>}
        {children}
      </a>
    )
  }

  return (
    <button className={base} {...props}>
      {icon && <span className="w-4 h-4">{icon}</span>}
      {children}
    </button>
  )
}
