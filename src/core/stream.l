;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;

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
     (:if (core:%streamp stream)
          ((:lambda (form)
             (:if (mu:eq form core:%read-list-eol%)
                  (core:raise stream 'core:read "unmatched close parenthesis")
                  (:if (mu:eq core:%eof% form)
                       (:if eof-error
                            (core:raise stream 'core:read "unexpected end of file")
                            eof-value)
                       form)))
           (core:%read stream))
          (core:raise stream 'core:read "not a stream"))))

(mu:intern core "write"
   (:lambda (object escape stream)
     (:if (core:%streamp stream)
          (:if (core:logand (core:vectorp object) (core:%core-type-p object))
               (core:%vector-write object escape stream)
               (mu:write object escape stream))
          (core:raise stream 'core:write "not a stream"))))
