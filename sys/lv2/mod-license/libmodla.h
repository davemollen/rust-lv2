/*
  Copyright 2016 Robin Gareus <robin@gareus.org>
  Copyright 2016 Filipe Coelho <falktx@falktx.com>
  Copyright 2018 MOD Devices GmbH

  Permission to use, copy, modify, and/or distribute this software for any
  purpose with or without fee is hereby granted, provided that the above
  copyright notice and this permission notice appear in all copies.

  THIS SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
  WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
  ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
  WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
  ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
  OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

#ifndef MOD_LICENSE_IMPL_H
#define MOD_LICENSE_IMPL_H

#ifdef __cplusplus
extern "C"
{
#endif

#include "mod-license.h"

#include <stdbool.h>

#pragma GCC visibility push(hidden)

/**
 * NOTE: You need to store a local 'uint32_t run_count' variable on your plugin,
 *       initialized with value 0.
 */

/** Check license file for a specific uri (plugin or collection).
 *
 * Must be called at instantiate(), one time for each license uri.
 *
 * Returns true if a valid license was found or host doesn't support licensing API.
 * (so that you can stop checking for other license uris)
 */
bool mod_license_check(const LV2_Feature* const* features, const char* license_uri);

/** Begin time calculations for unlicensed silence.
 *
 * Must be called at the beginning of each run().
 * This counts samples (time) to later decide if silence needs to be injected.
 *
 * Returned value must be stored in the local 'run_count'.
 */
uint32_t mod_license_run_begin(uint32_t run_count, uint32_t n_samples);

/** DEPRECATED
 * 
 * Before version 1.2 we used noise. We changed it, because even noise
 * at low levels can grow to maximum level through the signal
 * chain. This may damage speakers and hearing.
 *
 * We keep the function signature to not break compilation, since
 * programmers happen to fork e.g. old versions of DPF which make use
 * of `mod_license_run_noise`.
 */
void mod_license_run_noise(uint32_t run_count, float* buf, uint32_t n_samples, uint32_t chn)
  __attribute__ ((deprecated));
  
/** Inject silence into output buffers if unlicensed.
 *
 * Must be called at the end of each run(), for all audio output buffers.
 * Call this function on each buffer, using @a chn to specify the index offset.
 */
void mod_license_run_silence(uint32_t run_count, float* buf, uint32_t n_samples, uint32_t chn);

/** Get the LV2 interface for the MOD license API.
 *
 * Must be called at the end of your lv2 plugin extension_data.
 */
const void* mod_license_interface(const char* uri);

/** Return the version of modla library.
 *
 */
const char* mod_license_version(void);

#pragma GCC visibility pop

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif // MOD_LICENSE_IMPL_H
