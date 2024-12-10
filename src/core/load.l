;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loader
;;;

;;;
;;; load-file
;;;
(mu:intern core "load-file"
   (:lambda (path)
     (:if (core:stringp path)
          ((:lambda (stream)
             (mu:fix
              (:lambda (loop)
                (:if (mu:eq loop core:%eof%)
                     core:%eof%
                     ((:lambda (form)
                        (:if (mu:eq form core:%eof%)
                             core:%eof%
                             ((:lambda ()
                                (mu:eval (core:compile form))
                                (core:null loop)))))
                      (core:read stream () core:%eof%))))
              ()))
           (mu:open :file :input path :t))
          (core:raise path 'core:load-file "not a file path"))
     :t))
