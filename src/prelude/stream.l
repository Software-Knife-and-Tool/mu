;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;
(crux:intern prelude "streamp" (:lambda (t) (crux:eq :stream (crux:type-of t))))

;;;
;;; utilities
;;;
(crux:intern prelude "%write-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         crux:*standard-output*
         (:if (prelude:streamp designator)
              designator
              (prelude:raise designator
                          'prelude:%write-stream-designator
                          "not a stream designator")))))

(crux:intern prelude "%read-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         crux:*standard-input*
         (:if (crux:eq :t designator)
              crux:*standard-input*
              (:if (prelude:streamp designator)
                   designator
                   (prelude:raise designator
                               'prelude:%read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(crux:intern prelude "make-string-stream"
   (:lambda (dir init)
      (:if (prelude:keywordp dir)
           (:if (prelude:stringp init)
                (:if (crux:eq dir :input)
                     (crux:open :string :input init)
                     (:if (crux:eq dir :output)
                          (crux:open :string :output init)
                          (:if (crux:eq dir :bidir)
                               (crux:open :string :bidir init)
                               (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))))
                (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))
           (prelude:raise init 'prelude:make-string-stream "not a string initializer"))))

(crux:intern prelude "open"
  (:lambda (dir path)
     (:if (prelude:stringp path)
          (:if (prelude:keywordp dir)
               (:if (crux:eq dir :input)
                    (crux:open :file :input path)
                    (:if (crux:eq dir :output)
                         (crux:open :file :output path)
                         (prelude:raise dir 'prelude:open "not a direction keyword")))
               (prelude:raise path 'prelude:open "not a path string"))
          (prelude:raise dir 'prelude:open "not a direction keyword"))))

(crux:intern prelude "close"
   (:lambda (stream)
      (:if (prelude:streamp stream)
           (crux:close stream)
           (prelude:raise stream 'prelude:close "not a stream"))))

(crux:intern prelude "with-open-stream"
   (:lambda (type dir specifier fn)
     (:if (prelude:functionp fn)
          (:if (prelude:stringp specifier)
               (:if (prelude:%orf (crux:eq :file type)
                               (crux:eq :string type))
                    (:if (prelude:%orf (crux:eq :input dir)
                                    (crux:eq :output dir))
                         ((:lambda (stream)
                             (crux:unwind-protect
                              (:lambda (ex) (crux:close stream) ex)
                              (:lambda () (prelude:apply fn `(,stream)))))
                          (crux:open type dir specifier))
                         (prelude:raise dir 'prelude:with-open-stream "not a direction keyword"))
                    (prelude:raise type 'prelude:with-open-stream "not a type keyword"))
               (prelude:raise path 'prelude:with-open-stream "not a stream specifier"))
          (prelude:raise dir 'prelude:with-open-stream "not a function"))))

;;;
;;; chars, bytes, and unread
;;;
(crux:intern prelude "write-char"
   (:lambda (ch designator)
     (crux:write-char ch (prelude:%write-stream-designator designator))))

(crux:intern prelude "write-byte"
   (:lambda (byte designator)
     (crux:write-byte byte (prelude:%write-stream-designator designator))))

(crux:intern prelude "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (crux:read-char stream error-eofp eof-value))
     (prelude:%read-stream-designator designator))))

(crux:intern prelude "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (crux:read-byte stream error-eofp eof-value))
      (prelude:%read-stream-designator designator))))

(crux:intern prelude "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (crux:unread-char
        (prelude:read-char stream error-eofp eof-value)
        stream))
     (prelude:%read-stream-designator designator))))

(crux:intern prelude "unread-char"
   (:lambda (ch designator)
     (:if (prelude:charp ch)
          (crux:unread-char ch (prelude:%write-stream-designator designator))
          (prelude:raise ch 'prelude:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(crux:intern prelude "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (crux:eq prelude:%eof% form)
                (:if eof-error
                     (prelude:raise stream 'prelude:read "early end of file")
                     eof-value)
                (:if (crux:eq form prelude:%read-list-eol%)
                     (prelude:raise stream 'prelude:read "unmatched close parenthesis")
                     form)))
         (prelude:%read stream)))
      (prelude:%read-stream-designator designator))))

(crux:intern prelude "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (crux:eq value prelude:%eof%)
                eof-value
                (crux:get-string line)))              
         (crux:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (crux:eq ch prelude:%eof%)
                    prelude:%eof%
                    (:if (crux:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (crux:write-char ch line)
                            (prelude:null loop))))))
             (prelude:read-char stream eof-error-p prelude:%eof%)))
         ())))
      (crux:open :string :output ""))))

(crux:intern prelude "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (prelude:%andf (prelude:vectorp object) (prelude:%prelude-type-p object))
               (prelude:%vector-write object escape stream)
               (crux:write object escape stream)))
      (prelude:%read-stream-designator designator))))
