"use client"
import { useEffect } from 'react'
import { useRouter } from 'next/navigation'

export default function LegacyAddRedirect() {
  const router = useRouter()
  useEffect(() => { router.replace('/linalg/binary') }, [router])
  return null
}
