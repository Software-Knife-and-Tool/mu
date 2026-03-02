;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; arm-codegen
;;;

;;;
;;; namespace
;;;
(mu:make-namespace :arm)

(prelude:load "/opt/system-lisp/mu/arm-codegen/as.l" ())
(prelude:load "/opt/system-lisp/mu/arm-codegen/armv8-aarch64.l" ())
(prelude:load "/opt/system-lisp/mu/arm-codegen/compiler.l" ())
(prelude:load "/opt/system-lisp/mu/arm-codegen/elf.l" ())
(prelude:load "/opt/system-lisp/mu/arm-codegen/loader.l" ())
