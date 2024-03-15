;;;
;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;;  mu runtime environment
;;;

#|
(core:load "/opt/mu/mu/mu/mu.l" ())

(mu:eval (core:compile '(defconst mu::require-path "/opt/mu/lib")))

(core:in-namespace (mu:ns "user" (mu:map-ns "mu")))

(:if (core:zerop (system:system "stat ~/.mu > /dev/null 2>&1"))
     (core:load "~/.mu" () ())
     ())

(core:mapc
 (:lambda (fn) (core:load fn :t ()))
 (mu:read (mu:open :string :input (system:getenv "MU_LOAD_LIST"))))

(mu:intern (mu:find-ns "mu") :intern "mu-version" "0.0.1")

(mu:repl (mu:find-ns "user"))
|#
