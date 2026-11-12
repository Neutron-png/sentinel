import { useEffect, useState } from 'react'
import * as api from '../lib/bridge'
import { useApp } from '../state'
import { Tabs } from '../components/Tabs'

export function Settings({ theme, setTheme }: { theme: 'dark' | 'light'; setTheme: (t: 'dark' | 'light') => void }) {
  const app = useApp()
  const [tor, setTor] = useState(app.proxy?.tor_enabled ?? false)
  const [addr, setAddr] = useState(app.proxy?.tor_addr ?? '127.0.0.1:9050')
  const [busy, setBusy] = useState(false)
  const [saved, setSaved] = useState(false)
  const [error, setError] = useState('')

  useEffect(() => {
    if (app.proxy) { setTor(app.proxy.tor_enabled); setAddr(app.proxy.tor_addr) }
  }, [app.proxy])

  // The address only matters when routing is on.
  const addrOk = !tor || /^[\w.-]+:\d{2,5}$/.test(addr.trim())
  const dirty = !!app.proxy && (tor !== app.proxy.tor_enabled || addr.trim() !== app.proxy.tor_addr)

  const apply = async () => {
    if (!addrOk) return
    setBusy(true)
    setError('')
    try {
      app.setProxy(await api.setProxyMode(tor, addr.trim()))
      setSaved(true)
      setTimeout(() => setSaved(false), 2400)
    } catch (e) { setError(String(e).replace(/^Error:[ ]*/, '')) } finally { setBusy(false) }
  }

  return (
    <div className="settings">
      <h1 className="h1">Settings</h1>

      <section className="set-group" aria-labelledby="set-proxy">
        <h2 className="h3" id="set-proxy">Proxy</h2>
        <div className="set-row">
          <div>
            <div className="set-title">Listening address</div>
            <p className="note">Point the browser at this address. The port is fixed in this build.</p>
          </div>
          <code className="mono set-value">127.0.0.1:{app.proxy?.port ?? 8080}</code>
        </div>
        <div className="set-row">
          <div>
            <label className="set-title" htmlFor="set-tor">Route outbound traffic through SOCKS5</label>
            <p className="note">Proxy, Repeater, Scanner and crawler all use it. DNS resolves through the SOCKS server too (socks5h). TLS verification is relaxed while it is on.</p>
          </div>
          <label className="switch">
            <input id="set-tor" type="checkbox" role="switch" checked={tor} onChange={e => setTor(e.target.checked)} />
            <span className="switch-track" aria-hidden="true"><span className="switch-thumb" /></span>
          </label>
        </div>
        <div className="set-row">
          <div className="field grow">
            <label htmlFor="set-addr">SOCKS5 address</label>
            <input
              id="set-addr"
              className="mono"
              value={addr}
              onChange={e => setAddr(e.target.value)}
              aria-invalid={!addrOk}
              aria-describedby="set-addr-help"
              spellCheck={false}
            />
            <p id="set-addr-help" className={'help' + (addrOk ? '' : ' help-bad')}>
              {addrOk ? 'Tor listens on 127.0.0.1:9050 by default. Sentinel does not start Tor for you.' : 'Use host:port, for example 127.0.0.1:9050.'}
            </p>
          </div>
        </div>
        <div className="set-actions">
          <button type="button" className="btn btn-primary" onClick={apply} disabled={!dirty || !addrOk || busy}>
            {busy ? 'Restarting proxy' : saved ? 'Applied' : 'Apply and restart proxy'}
          </button>
          {dirty && <span className="note">Restarting drops open connections through the proxy.</span>}
        </div>
        {error && <p className="inline-error inline-error-flush" role="alert"><strong>Proxy not restarted.</strong> {error} The previous settings are still active.</p>}
      </section>

      <section className="set-group" aria-labelledby="set-look">
        <h2 className="h3" id="set-look">Appearance</h2>
        <div className="set-row">
          <div>
            <div className="set-title">Theme</div>
            <p className="note">Dark is tuned for long sessions. Light is tuned for daylight and screen sharing.</p>
          </div>
          <Tabs label="Theme" value={theme} onChange={setTheme} options={[{ id: 'dark', label: 'Dark' }, { id: 'light', label: 'Light' }]} />
        </div>
      </section>
    </div>
  )
}
