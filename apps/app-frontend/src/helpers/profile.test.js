import { describe, it, expect, vi, beforeEach } from 'vitest'
import { create, remove, get } from './profile'

// Mock the Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('profile helpers', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  describe('create', () => {
    it('trims whitespace from profile name', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      invoke.mockResolvedValue({ success: true })

      const profileName = '  My Profile  '
      await create(profileName, '1.20.1', 'Vanilla', 'latest', '/path/to/icon', false)

      expect(invoke).toHaveBeenCalledWith('plugin:profile-create|profile_create', {
        name: 'My Profile', // Should be trimmed
        gameVersion: '1.20.1',
        modloader: 'Vanilla',
        loaderVersion: 'latest',
        iconPath: '/path/to/icon',
        skipInstall: false,
      })
    })

    it('calls the correct Tauri command', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      invoke.mockResolvedValue({ success: true })

      await create('TestProfile', '1.19.4', 'Fabric', 'stable', '/icon.png', true)

      expect(invoke).toHaveBeenCalledWith('plugin:profile-create|profile_create', {
        name: 'TestProfile',
        gameVersion: '1.19.4',
        modloader: 'Fabric',
        loaderVersion: 'stable',
        iconPath: '/icon.png',
        skipInstall: true,
      })
    })
  })

  describe('remove', () => {
    it('calls the remove command with correct path', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      invoke.mockResolvedValue(undefined)

      await remove('/path/to/profile')

      expect(invoke).toHaveBeenCalledWith('plugin:profile|profile_remove', {
        path: '/path/to/profile',
      })
    })
  })

  describe('get', () => {
    it('retrieves profile data', async () => {
      const { invoke } = await import('@tauri-apps/api/core')
      const mockProfile = {
        name: 'MyProfile',
        path: '/profiles/MyProfile',
        game_version: '1.20.1',
      }
      invoke.mockResolvedValue(mockProfile)

      const result = await get('/profiles/MyProfile')

      expect(invoke).toHaveBeenCalledWith('plugin:profile|profile_get', {
        path: '/profiles/MyProfile',
      })
      expect(result).toEqual(mockProfile)
    })
  })
})
