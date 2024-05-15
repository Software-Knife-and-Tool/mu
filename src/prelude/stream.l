;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;
(core:intern prelude "streamp" (:lambda (t) (core:eq :stream (core:type-of t))))

;;;
;;; utilities
;;;
(core:intern prelude "%write-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         core:*standard-output*
         (:if (prelude:streamp designator)
              designator
              (prelude:raise designator
                          'prelude:%write-stream-designator
                          "not a stream designator")))))

(core:intern prelude "%read-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         core:*standard-input*
         (:if (core:eq :t designator)
              core:*standard-input*
              (:if (prelude:streamp designator)
                   designator
                   (prelude:raise designator
                               'prelude:%read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(core:intern prelude "make-string-stream"
   (:lambda (dir init)
      (:if (prelude:keywordp dir)
           (:if (prelude:stringp init)
                (:if (core:eq dir :input)
                     (core:open :string :input init)
                     (:if (core:eq dir :output)
                          (core:open :string :output init)
                          (:if (core:eq dir :bidir)
                               (core:open :string :bidir init)
                               (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))))
                (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))
           (prelude:raise init 'prelude:make-string-stream "not a string initializer"))))

(core:intern prelude "open"
  (:lambda (dir path)
     (:if (prelude:stringp path)
          (:if (prelude:keywordp dir)
               (:if (core:eq dir :input)
                    (core:open :file :input path)
                    (:if (core:eq dir :output)
                         (core:open :file :output path)
                         (prelude:raise dir 'prelude:open "not a direction keyword")))
               (prelude:raise path 'prelude:open "not a path string"))
          (prelude:raise dir 'prelude:open "not a direction keyword"))))

(core:intern prelude "close"
   (:lambda (stream)
      (:if (prelude:streamp stream)
           (core:close stream)
           (prelude:raise stream 'prelude:close "not a stream"))))

(core:intern prelude "with-open-stream"
   (:lambda (type dir specifier fn)
     (:if (prelude:functionp fn)
          (:if (prelude:stringp specifier)
               (:if (prelude:%orf (core:eq :file type)
                               (core:eq :string type))
                    (:if (prelude:%orf (core:eq :input dir)
                                    (core:eq :output dir))
                         ((:lambda (stream)
                             (core:unwind-protect
                              (:lambda (ex) (core:close stream) ex)
                              (:lambda () (prelude:apply fn `(,stream)))))
                          (core:open type dir specifier))
                         (prelude:raise dir 'prelude:with-open-stream "not a direction keyword"))
                    (prelude:raise type 'prelude:with-open-stream "not a type keyword"))
               (prelude:raise path 'prelude:with-open-stream "not a stream specifier"))
          (prelude:raise dir 'prelude:with-open-stream "not a function"))))

;;;
;;; chars, bytes, and unread
;;;
(core:intern prelude "write-char"
   (:lambda (ch designator)
     (core:write-char ch (prelude:%write-stream-designator designator))))

(core:intern prelude "write-byte"
   (:lambda (byte designator)
     (core:write-byte byte (prelude:%write-stream-designator designator))))

(core:intern prelude "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (core:read-char stream error-eofp eof-value))
     (prelude:%read-stream-designator designator))))

(core:intern prelude "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (core:read-byte stream error-eofp eof-value))
      (prelude:%read-stream-designator designator))))

(core:intern prelude "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (core:unread-char
        (prelude:read-char stream error-eofp eof-value)
        stream))
     (prelude:%read-stream-designator designator))))

(core:intern prelude "unread-char"
   (:lambda (ch designator)
     (:if (prelude:charp ch)
          (core:unread-char ch (prelude:%write-stream-designator designator))
          (prelude:raise ch 'prelude:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(core:intern prelude "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (core:eq prelude:%eof% form)
                (:if eof-error
                     (prelude:raise stream 'prelude:read "early end of file")
                     eof-value)
                (:if (core:eq form prelude:%read-list-eol%)
                     (prelude:raise stream 'prelude:read "unmatched close parenthesis")
                     form)))
         (prelude:%read stream)))
      (prelude:%read-stream-designator designator))))

(core:intern prelude "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (core:eq value prelude:%eof%)
                eof-value
                (core:get-string line)))              
         (core:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (core:eq ch prelude:%eof%)
                    prelude:%eof%
                    (:if (core:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (core:write-char ch line)
                            (prelude:null loop))))))
             (prelude:read-char stream eof-error-p prelude:%eof%)))
         ())))
      (core:open :string :output ""))))

(core:intern prelude "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (prelude:%andf (prelude:vectorp object) (prelude:%prelude-type-p object))
               (prelude:%vector-write object escape stream)
               (core:write object escape stream)))
      (prelude:%read-stream-designator designator))))
