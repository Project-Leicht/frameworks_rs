/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "shared.rsh"
#include "rs_graphics.rsh"
rs_sampler minification;
rs_sampler magnification;
rs_sampler wrapS;
rs_sampler wrapT;
rs_sampler anisotropy;

static bool test_sampler_getters() {
    bool failed = false;

    _RS_ASSERT(rsSamplerGetMagnification(minification) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetMinification(minification) == RS_SAMPLER_LINEAR_MIP_LINEAR);
    _RS_ASSERT(rsSamplerGetWrapS(minification) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetWrapT(minification) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetAnisotropy(minification) == 1.0f);

    _RS_ASSERT(rsSamplerGetMagnification(magnification) == RS_SAMPLER_LINEAR);
    _RS_ASSERT(rsSamplerGetMinification(magnification) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetWrapS(magnification) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetWrapT(magnification) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetAnisotropy(magnification) == 1.0f);

    _RS_ASSERT(rsSamplerGetMagnification(wrapS) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetMinification(wrapS) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetWrapS(wrapS) == RS_SAMPLER_WRAP);
    _RS_ASSERT(rsSamplerGetWrapT(wrapS) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetAnisotropy(wrapS) == 1.0f);

    _RS_ASSERT(rsSamplerGetMagnification(wrapT) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetMinification(wrapT) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetWrapS(wrapT) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetWrapT(wrapT) == RS_SAMPLER_WRAP);
    _RS_ASSERT(rsSamplerGetAnisotropy(wrapT) == 1.0f);

    _RS_ASSERT(rsSamplerGetMagnification(anisotropy) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetMinification(anisotropy) == RS_SAMPLER_NEAREST);
    _RS_ASSERT(rsSamplerGetWrapS(anisotropy) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetWrapT(anisotropy) == RS_SAMPLER_CLAMP);
    _RS_ASSERT(rsSamplerGetAnisotropy(anisotropy) == 8.0f);

    if (failed) {
        rsDebug("test_sampler_getters FAILED", 0);
    }
    else {
        rsDebug("test_sampler_getters PASSED", 0);
    }

    return failed;
}

void sampler_test() {
    bool failed = false;
    failed |= test_sampler_getters();

    if (failed) {
        rsSendToClientBlocking(RS_MSG_TEST_FAILED);
    }
    else {
        rsSendToClientBlocking(RS_MSG_TEST_PASSED);
    }
}

