import { describe, it, expect } from 'vitest'
import { releaseColor } from './utils'

describe('releaseColor', () => {
  it('returns green for release', () => {
    expect(releaseColor('release')).toBe('green')
  })

  it('returns orange for beta', () => {
    expect(releaseColor('beta')).toBe('orange')
  })

  it('returns red for alpha', () => {
    expect(releaseColor('alpha')).toBe('red')
  })

  it('returns empty string for unknown release type', () => {
    expect(releaseColor('unknown')).toBe('')
  })

  it('returns empty string for undefined', () => {
    expect(releaseColor(undefined)).toBe('')
  })
})
