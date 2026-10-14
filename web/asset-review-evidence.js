export function plain(value) {
  if (typeof value === 'bigint') return value.toString();
  if (value instanceof Map) return Object.fromEntries([...value].map(([key, item]) => [key, plain(item)]));
  if (Array.isArray(value)) return value.map(plain);
  if (ArrayBuffer.isView(value)) return Array.from(value, plain);
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, plain(item)]));
  return value;
}

function literal(value) {
  if (value === null || value === undefined) return 'null';
  if (typeof value === 'string' || typeof value === 'boolean') return JSON.stringify(value);
  if (typeof value === 'number') {
    if (!Number.isFinite(value) || (Number.isInteger(value) && !Number.isSafeInteger(value))) throw new Error('Non-finite or inexact numeric input cannot cross the Vibe boundary; encode u64 identifiers as strings.');
    const encoded = String(value);
    return Number.isInteger(value) && !encoded.includes('e') ? `${encoded}.0` : encoded;
  }
  if (Array.isArray(value)) return `[${value.map(literal).join(',')}]`;
  if (typeof value === 'object') return `{wire_record:true,fields:[${Object.entries(value).map(([key, item]) => `{key:${JSON.stringify(key)},value:${literal(item)}}`).join(',')}]}`;
  throw new Error('Unsupported browser input at the Vibe boundary.');
}

function unwire(value) {
  if (Array.isArray(value)) return value.map(unwire);
  if (value && typeof value === 'object') {
    if (value.wire_record === true) return Object.fromEntries(value.fields.map(field => [field.key, unwire(field.value)]));
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, unwire(item)]));
  }
  return value;
}

export function invokeVibe(module, source, name, input) {
  if (!/^review_[a-z_]+$/.test(name)) throw new Error('Unknown Vibe review entry point.');
  if (typeof module.eval_program_src !== 'function') throw new Error('The package lacks the Vibe program evaluator; no JavaScript substitute is available.');
  const result = plain(module.eval_program_src(`${source}\nfn main() { return ${name}(${literal(input)}); }`));
  if (result?.ok !== true) throw new Error(`VibeScript: ${JSON.stringify(result?.error || result)}`);
  const value = unwire(result.value);
  if (value?.ok === false) throw new Error(value.detail || 'VibeScript refused this operation.');
  return value;
}

export async function sha256(bytes) {
  if (!globalThis.crypto?.subtle) throw new Error('Web Crypto requires localhost or HTTPS.');
  return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', bytes)), byte => byte.toString(16).padStart(2, '0')).join('');
}

export async function fetchBounded(path, limit, signal) {
  const url = new URL(path, location.href);
  if (url.origin !== location.origin) throw new Error('Only same-origin browser transport is permitted.');
  const response = await fetch(url, { signal, cache: 'no-store', redirect: 'error' });
  if (!response.ok) throw new Error(`${url.pathname}: HTTP ${response.status}`);
  const reader = response.body?.getReader();
  if (!reader) throw new Error('Bounded browser streaming is unavailable.');
  const chunks = [];
  let length = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      length += value.byteLength;
      if (length > limit) throw new Error(`${url.pathname}: transfer bound exceeded.`);
      chunks.push(value);
    }
  } catch (error) {
    await reader.cancel().catch(() => {});
    throw error;
  } finally {
    reader.releaseLock();
  }
  const bytes = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.byteLength; }
  return bytes;
}

export function downloadJson(record) {
  downloadJsonNamed(record, 'asset-review.json');
}

export function downloadJsonNamed(record, filename = 'asset-review.json') {
  const url = URL.createObjectURL(new Blob([JSON.stringify(record, null, 2)], { type: 'application/json' }));
  const anchor = document.createElement('a');
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}

export async function copyToClipboard(content) {
  const textToCopy = typeof content === 'string' ? content : JSON.stringify(plain(content), null, 2);
  if (navigator.clipboard?.writeText) {
    await navigator.clipboard.writeText(textToCopy);
    return true;
  }
  return false;
}

/**
 * Evaluates Jakob & Hanika (2019) bounded spectral reflectance at a specific wavelength in nm.
 * Wavelength range: [380, 780] nm.
 * Returns reflectance R in [0.0, 1.0].
 */
export function evaluateSpectralReflectance(c0, c1, c2, wavelengthNm) {
  const u = Math.max(0.0, Math.min(1.0, (wavelengthNm - 380.0) / 400.0));
  const f = (c0 * u * u) + (c1 * u) + c2;
  const denom = Math.sqrt(1.0 + (f * f));
  return 0.5 + 0.5 * (f / denom);
}

/**
 * Generates a synthetic acoustic impulse audition shaped by 3-band anchor frequencies.
 * Emulates the Direct Form II Transposed (DF2T) AudioWorklet filter.
 */
let sharedAudioCtx = null;
export function playAcousticImpulse(lowHz = 200, midHz = 1000, highHz = 8000, pan = 0.0) {
  try {
    const AudioContextClass = window.AudioContext || window.webkitAudioContext;
    if (!AudioContextClass) return false;
    if (!sharedAudioCtx || sharedAudioCtx.state === 'closed') {
      sharedAudioCtx = new AudioContextClass();
    }
    if (sharedAudioCtx.state === 'suspended') {
      sharedAudioCtx.resume();
    }
    const ctx = sharedAudioCtx;
    const now = ctx.currentTime;

    // Excitation: 20ms burst
    const bufferSize = Math.floor(ctx.sampleRate * 0.04);
    const buffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < bufferSize; i++) {
      // Noise burst with exponential decay + DC offset protection
      data[i] = ((Math.random() * 2 - 1) * Math.exp(-i / (ctx.sampleRate * 0.008))) + 1e-12;
    }

    const source = ctx.createBufferSource();
    source.buffer = buffer;

    // 3-Band Parametric Biquad Stages
    const lowFilter = ctx.createBiquadFilter();
    lowFilter.type = 'lowshelf';
    lowFilter.frequency.setValueAtTime(Math.max(20, Math.min(lowHz, 1000)), now);
    lowFilter.gain.setValueAtTime(3.0, now);

    const midFilter = ctx.createBiquadFilter();
    midFilter.type = 'peaking';
    midFilter.frequency.setValueAtTime(Math.max(200, Math.min(midHz, 5000)), now);
    midFilter.Q.setValueAtTime(1.2, now);
    midFilter.gain.setValueAtTime(-2.0, now);

    const highFilter = ctx.createBiquadFilter();
    highFilter.type = 'highshelf';
    highFilter.frequency.setValueAtTime(Math.max(2000, Math.min(highHz, 16000)), now);
    highFilter.gain.setValueAtTime(-6.0, now);

    // Spatial Panner
    let panner = null;
    if (ctx.createStereoPanner) {
      panner = ctx.createStereoPanner();
      panner.pan.setValueAtTime(Math.max(-1.0, Math.min(1.0, pan)), now);
    }

    // Gain envelope
    const masterGain = ctx.createGain();
    masterGain.gain.setValueAtTime(0.3, now);
    masterGain.gain.exponentialRampToValueAtTime(0.0001, now + 0.35);

    // Wiring
    source.connect(lowFilter);
    lowFilter.connect(midFilter);
    midFilter.connect(highFilter);
    if (panner) {
      highFilter.connect(panner);
      panner.connect(masterGain);
    } else {
      highFilter.connect(masterGain);
    }
    masterGain.connect(ctx.destination);

    source.start(now);
    source.stop(now + 0.4);
    return true;
  } catch (err) {
    console.warn('Audio impulse audition unavailable:', err);
    return false;
  }
}
