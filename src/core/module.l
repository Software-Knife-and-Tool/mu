;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; modules
;;;

;;;
;;; require
;;;
(mu:intern core "require"
   (:lambda (module)
     (:if (core:stringp module)
          ((:lambda ()
             (:if (mu:find core:%modules% module)
                  module
                  ((:lambda ()
                     (core:load-file (core:%format () "/opt/mu/modules/~A/~A.l" `(,module ,module)))
                     ((:lambda (file-list)
                        (core:%mapc
                         (:lambda (file-name)
                           (core:load-file (core:%format () "/opt/mu/modules/~A/~A" `(,module ,file-name))))
                         file-list)
                        :t)
                      (mu:symbol-value (mu:find core:%modules% module))))))))
          (core:raise module 'core:require "is not a provided module"))))

;;;
;;; provide
;;;
(mu:intern core "provide"
   (:lambda (module file-list)
     (:if (core:stringp module)
          (mu:intern core:%modules% module file-list)
          (core:raise module 'core:provide "is not a module name"))))

;;;
;;; modules
;;;
(mu:intern core "modules"
   (:lambda ()
     (core:%mapcar mu:symbol-name (mu:namespace-symbols core:%modules%))))
