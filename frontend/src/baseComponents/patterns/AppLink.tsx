"use client";
import Link from 'next/link';
import React from 'react';

export type AppLinkProps = {
  href: string;
  children: React.ReactNode;
  variant?: 'inline' | 'list';
  chevron?: boolean;
  className?: string;
  style?: React.CSSProperties;
};

export const AppLink: React.FC<AppLinkProps> = ({ href, children, variant = 'list', chevron = true, className, style }) => {
  if (variant === 'inline') {
    return (
      <Link
        href={href}
        className={["app-link-inline", className].filter(Boolean).join(' ')}
        style={style}
      >
        {children}
      </Link>
    );
  }

  return (
    <Link
      href={href}
      className={["nav-link", className].filter(Boolean).join(' ')}
      style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', ...style }}
    >
      <span>{children}</span>
      {chevron && <span aria-hidden="true" style={{ opacity: 0.7 }}>→</span>}
    </Link>
  );
};

export default AppLink;
