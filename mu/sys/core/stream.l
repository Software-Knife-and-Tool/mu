;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;

;;;
;;; load
;;;
(mu:intern core "load"
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
                             (core:%prog2
                                (mu:eval (core:compile form))
                                (core:null loop))))
                      (core:read stream () core:%eof%))))
              ()))
           (mu:open :file :input path :t))
          (mu:raise path 'core:load :type))
     :t))

;;;
;;; functions
;;;
(mu:intern core "%peek-char"
   (:lambda (stream error-eofp eof-value)
    (mu:unread-char
     (mu:read-char stream error-eofp eof-value)
     stream)))

;;;
;;; read/write
;;;
(mu:intern core "read"
   (:lambda (stream eof-error eof-value)
     (:if (core:streamp stream)
          ((:lambda (form)
             (:if (mu:eq form core:%read-list-eol%)
                  (mu:raise stream 'core:read :syntax)
                  (:if (mu:eq core:%eof% form)
                       (:if eof-error
                            (mu:raise stream 'core:read :eof)
                            eof-value)
                       form)))
           (core:%read stream))
          (mu:raise stream 'core:read :type))))

(mu:intern core "write"
   (:lambda (form escape stream)
     (:if (core:streamp stream)
          (:if (core:%and (core:vectorp form) (core:%typep form))
               (core:%vector-write form escape stream)
               (:if (mu:eq :symbol (mu:type-of form))
                    (:if (core:null escape)
                         ((:lambda (ns name)
                            (:if (mu:eq core:%reader/ ns)
                                 (mu:write name () stream)
                                 (mu:write form () stream)))
                            (mu:symbol-namespace form)
                            (mu:symbol-name form))
                         (mu:write form escape stream))
               (mu:write form escape stream)))
          (mu:raise stream 'core:write :type))))
