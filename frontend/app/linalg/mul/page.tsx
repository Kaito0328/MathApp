"use client"
import { useEffect } from 'react'
import { useRouter } from 'next/navigation'

export default function LegacyMulRedirect() {
  const router = useRouter()
  useEffect(() => { router.replace('/linalg/binary') }, [router])
  return null
}
