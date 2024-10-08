;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; loaders
;;;

;;;
;;; %load-file
;;;
(mu:intern core "%load-file"
   (:lambda (path)
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
               (core:%read stream))))
        ()))
      (mu:open :file :input path))
     ()))
