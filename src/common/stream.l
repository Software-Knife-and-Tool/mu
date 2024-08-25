;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;

;;;
;;; implementation
;;;
(mu:intern common "write-stream-designator"
  (:lambda (designator)
    (:if (common:null designator)
         mu:*standard-output*
         (:if (common:streamp designator)
              designator
              (common:raise designator
                          'common:write-stream-designator
                          "not a stream designator")))))

(mu:intern common "read-stream-designator"
  (:lambda (designator)
    (:if (common:null designator)
         mu:*standard-input*
         (:if (mu:eq :t designator)
              mu:*standard-input*
              (:if (common:streamp designator)
                   designator
                   (common:raise designator
                               'common:read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(mu:intern common "make-string-stream"
   (:lambda (dir init)
      (:if (common:keywordp dir)
           (:if (common:stringp init)
                (:if (mu:eq dir :input)
                     (mu:open :string :input init)
                     (:if (mu:eq dir :output)
                          (mu:open :string :output init)
                          (:if (mu:eq dir :bidir)
                               (mu:open :string :bidir init)
                               (common:raise dir 'common:make-string-stream "not a direction keyword"))))
                (common:raise dir 'common:make-string-stream "not a direction keyword"))
           (common:raise init 'common:make-string-stream "not a string initializer"))))

(mu:intern common "open"
  (:lambda (dir path)
     (:if (common:stringp path)
          (:if (common:keywordp dir)
               (:if (mu:eq dir :input)
                    (mu:open :file :input path)
                    (:if (mu:eq dir :output)
                         (mu:open :file :output path)
                         (common:raise dir 'common:open "not a direction keyword")))
               (common:raise path 'common:open "not a path string"))
          (common:raise dir 'common:open "not a direction keyword"))))

;;;
;;; functions
;;;
(mu:intern common "close"
   (:lambda (stream)
      (:if (common:streamp stream)
           (mu:close stream)
           (common:raise stream 'common:close "not a stream"))))

(mu:intern common "with-open-file"
   (:lambda (type dir specifier fn)
     (:if (common:functionp fn)
          (:if (common:stringp specifier)
               (:if (common:%orf (mu:eq :file type)
                               (mu:eq :string type))
                    (:if (common:%orf (mu:eq :input dir)
                                    (mu:eq :output dir))
                         ((:lambda (stream)
                             (mu:with-exception
                              (:lambda (ex) (mu:close stream) ex)
                              (:lambda () (common:%apply fn `(,stream)))))
                          (mu:open type dir specifier))
                         (common:raise dir 'common:with-open-file "not a direction keyword"))
                    (common:raise type 'common:with-open-file "not a type keyword"))
               (common:raise path 'common:with-open-file "not a stream specifier"))
          (common:raise dir 'common:with-open-file "not a function"))))

(mu:intern common "load-file"
   (:lambda (path)
     (common:with-open-file :file :input path
       (:lambda (stream)
         (mu:fix
             (:lambda (loop)
               (:if (mu:eq form common:%eof%)
                    loop
                    (common:null loop))
               (mu:eval (common:compile form)))))
       (common:read stream () common:%eof%))))

;;;
;;; chars, bytes, and unread
;;;
(mu:intern common "write-char"
   (:lambda (ch designator)
     (mu:write-char ch (common:write-stream-designator designator))))

(mu:intern common "write-byte"
   (:lambda (byte designator)
     (mu:write-byte byte (common:write-stream-designator designator))))

(mu:intern common "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:read-char stream error-eofp eof-value))
     (common:read-stream-designator designator))))

(mu:intern common "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (mu:read-byte stream error-eofp eof-value))
      (common:read-stream-designator designator))))

(mu:intern common "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:unread-char
        (common:read-char stream error-eofp eof-value)
        stream))
     (common:read-stream-designator designator))))

(mu:intern common "unread-char"
   (:lambda (ch designator)
     (:if (common:charp ch)
          (mu:unread-char ch (common:write-stream-designator designator))
          (common:raise ch 'common:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(mu:intern common "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (mu:eq common:%eof% form)
                (:if eof-error
                     (common:raise stream 'common:read "unexpected end of file")
                     eof-value)
                (:if (mu:eq form common:%read-list-eol%)
                     (common:raise stream 'common:read "unmatched close parenthesis")
                     form)))
         (common:%read stream)))
      (common:read-stream-designator designator))))

(mu:intern common "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (mu:eq value common:%eof%)
                eof-value
                (mu:get-string line)))              
         (mu:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (mu:eq ch common:%eof%)
                    common:%eof%
                    (:if (mu:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (mu:write-char ch line)
                            (common:null loop))))))
             (common:read-char stream eof-error-p common:%eof%)))
         ())))
      (mu:open :string :output ""))))

(mu:intern common "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (common:%andf (common:vectorp object) (common:%common-type-p object))
               (common:%vector-write object escape stream)
               (mu:write object escape stream)))
      (common:read-stream-designator designator))))
