;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;

;;;
;;; implementation
;;;
(mu:intern core "write-stream-designator"
  (:lambda (designator)
    (:if (core:null designator)
         mu:*standard-output*
         (:if (core:streamp designator)
              designator
              (core:raise designator
                          'core:write-stream-designator
                          "not a stream designator")))))

(mu:intern core "read-stream-designator"
  (:lambda (designator)
    (:if (core:null designator)
         mu:*standard-input*
         (:if (mu:eq :t designator)
              mu:*standard-input*
              (:if (core:streamp designator)
                   designator
                   (core:raise designator
                               'core:read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(mu:intern core "make-string-stream"
   (:lambda (dir init)
      (:if (core:keywordp dir)
           (:if (core:stringp init)
                (:if (mu:eq dir :input)
                     (mu:open :string :input init)
                     (:if (mu:eq dir :output)
                          (mu:open :string :output init)
                          (:if (mu:eq dir :bidir)
                               (mu:open :string :bidir init)
                               (core:raise dir 'core:make-string-stream "not a direction keyword"))))
                (core:raise dir 'core:make-string-stream "not a direction keyword"))
           (core:raise init 'core:make-string-stream "not a string initializer"))))

(mu:intern core "open"
  (:lambda (dir path)
     (:if (core:stringp path)
          (:if (core:keywordp dir)
               (:if (mu:eq dir :input)
                    (mu:open :file :input path)
                    (:if (mu:eq dir :output)
                         (mu:open :file :output path)
                         (core:raise dir 'core:open "not a direction keyword")))
               (core:raise path 'core:open "not a path string"))
          (core:raise dir 'core:open "not a direction keyword"))))

;;;
;;; functions
;;;
(mu:intern core "close"
   (:lambda (stream)
      (:if (core:streamp stream)
           (mu:close stream)
           (core:raise stream 'core:close "not a stream"))))

(mu:intern core "with-open-file"
   (:lambda (type dir specifier fn)
     (:if (core:functionp fn)
          (:if (core:stringp specifier)
               (:if (core:%orf (mu:eq :file type)
                               (mu:eq :string type))
                    (:if (core:%orf (mu:eq :input dir)
                                    (mu:eq :output dir))
                         ((:lambda (stream)
                             (mu:with-exception
                              (:lambda (ex) (mu:close stream) ex)
                              (:lambda () (core:%apply fn `(,stream)))))
                          (mu:open type dir specifier))
                         (core:raise dir 'core:with-open-file "not a direction keyword"))
                    (core:raise type 'core:with-open-file "not a type keyword"))
               (core:raise path 'core:with-open-file "not a stream specifier"))
          (core:raise dir 'core:with-open-file "not a function"))))

(mu:intern core "load-file"
   (:lambda (path)
     (core:with-open-file :file :input path
       (:lambda (stream)
         (mu:fix
             (:lambda (loop)
               (:if (mu:eq form core:%eof%)
                    loop
                    (core:null loop))
               (mu:eval (core:compile form)))))
       (core:read stream () core:%eof%))))

;;;
;;; chars, bytes, and unread
;;;
(mu:intern core "write-char"
   (:lambda (ch designator)
     (mu:write-char ch (core:write-stream-designator designator))))

(mu:intern core "write-byte"
   (:lambda (byte designator)
     (mu:write-byte byte (core:write-stream-designator designator))))

(mu:intern core "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:read-char stream error-eofp eof-value))
     (core:read-stream-designator designator))))

(mu:intern core "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (mu:read-byte stream error-eofp eof-value))
      (core:read-stream-designator designator))))

(mu:intern core "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:unread-char
        (core:read-char stream error-eofp eof-value)
        stream))
     (core:read-stream-designator designator))))

(mu:intern core "unread-char"
   (:lambda (ch designator)
     (:if (core:charp ch)
          (mu:unread-char ch (core:write-stream-designator designator))
          (core:raise ch 'core:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(mu:intern core "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (mu:eq core:%eof% form)
                (:if eof-error
                     (core:raise stream 'core:read "unexpected end of file")
                     eof-value)
                (:if (mu:eq form core:%read-list-eol%)
                     (core:raise stream 'core:read "unmatched close parenthesis")
                     form)))
         (core:%read stream)))
      (core:read-stream-designator designator))))

(mu:intern core "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (mu:eq value core:%eof%)
                eof-value
                (mu:get-string line)))              
         (mu:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (mu:eq ch core:%eof%)
                    core:%eof%
                    (:if (mu:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (mu:write-char ch line)
                            (core:null loop))))))
             (core:read-char stream eof-error-p core:%eof%)))
         ())))
      (mu:open :string :output ""))))

(mu:intern core "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (core:%andf (core:vectorp object) (core:%core-type-p object))
               (core:%vector-write object escape stream)
               (mu:write object escape stream)))
      (core:read-stream-designator designator))))
