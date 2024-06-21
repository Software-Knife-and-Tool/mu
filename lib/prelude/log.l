;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; logging
;;;
(mu:intern prelude "%logf"
   (:lambda (stream format-satring arg-list)
      (prelude:apply prelude:format `(,stream ,format-string ,arg-list))))

(mu:intern prelude "logf"
   (:lambda (env format-string arg-list)
      ((:lambda (stream)
          (prelude:apply prelude:format `(,stream ,format-string ,arg-list))))
      (prelude:%env-prop :log env)))

(mu:intern prelude "%open-log-file"
   (:lambda (file-name env)
      (mu:open :file :output file-name)))

;;;
;;; exception flavors
;;;
(mu:intern prelude "log-error"
   (:lambda (format-string value env)
      (prelude:logf env format-string `(,value))
      (prelude:error format-string value)))

(mu:intern prelude "log-raise"
   (:lambda (format-string value source reason env)
      (prelude:logf env format-string `(,value ,source ,reason ,env))
      (prelude:raise value source reason)))

(mu:intern prelude "log-warn"
   (:lambda (value message env)
      (prelude:logf env ";;; warning: ~S ~A~%" `(,value ,message))
      value))
