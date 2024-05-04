;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;
(lib:intern prelude "streamp" (:lambda (t) (lib:eq :stream (lib:type-of t))))

;;;
;;; utilities
;;;
(lib:intern prelude "%write-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         lib:standard-output
         (:if (prelude:streamp designator)
              designator
              (prelude:raise designator
                          'prelude:%write-stream-designator
                          "not a stream designator")))))

(lib:intern prelude "%read-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         lib:standard-input
         (:if (lib:eq :t designator)
              lib:standard-input
              (:if (prelude:streamp designator)
                   designator
                   (prelude:raise designator
                               'prelude:%read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(lib:intern prelude "make-string-stream"
   (:lambda (dir init)
      (:if (prelude:keywordp dir)
           (:if (prelude:stringp init)
                (:if (lib:eq dir :input)
                     (lib:open :string :input init)
                     (:if (lib:eq dir :output)
                          (lib:open :string :output init)
                          (:if (lib:eq dir :bidir)
                               (lib:open :string :bidir init)
                               (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))))
                (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))
           (prelude:raise init 'prelude:make-string-stream "not a string initializer"))))

(lib:intern prelude "open"
  (:lambda (dir path)
     (:if (prelude:stringp path)
          (:if (prelude:keywordp dir)
               (:if (lib:eq dir :input)
                    (lib:open :file :input path)
                    (:if (lib:eq dir :output)
                         (lib:open :file :output path)
                         (prelude:raise dir 'prelude:open "not a direction keyword")))
               (prelude:raise path 'prelude:open "not a path string"))
          (prelude:raise dir 'prelude:open "not a direction keyword"))))

(lib:intern prelude "close"
   (:lambda (stream)
      (:if (prelude:streamp stream)
           (lib:close stream)
           (prelude:raise stream 'prelude:close "not a stream"))))

(lib:intern prelude "with-open-stream"
   (:lambda (type dir specifier fn)
     (:if (prelude:functionp fn)
          (:if (prelude:stringp specifier)
               (:if (prelude:%orf (lib:eq :file type)
                               (lib:eq :string type))
                    (:if (prelude:%orf (lib:eq :input dir)
                                    (lib:eq :output dir))
                         ((:lambda (stream)
                             (lib:unwind-protect
                              (:lambda (ex) (lib:close stream) ex)
                              (:lambda () (prelude:apply fn `(,stream)))))
                          (lib:open type dir specifier))
                         (prelude:raise dir 'prelude:with-open-stream "not a direction keyword"))
                    (prelude:raise type 'prelude:with-open-stream "not a type keyword"))
               (prelude:raise path 'prelude:with-open-stream "not a stream specifier"))
          (prelude:raise dir 'prelude:with-open-stream "not a function"))))

;;;
;;; chars, bytes, and unread
;;;
(lib:intern prelude "write-char"
   (:lambda (ch designator)
     (lib:wr-char ch (prelude:%write-stream-designator designator))))

(lib:intern prelude "write-byte"
   (:lambda (byte designator)
     (lib:wr-byte byte (prelude:%write-stream-designator designator))))

(lib:intern prelude "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (lib:rd-char stream error-eofp eof-value))
     (prelude:%read-stream-designator designator))))

(lib:intern prelude "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (lib:rd-byte stream error-eofp eof-value))
      (prelude:%read-stream-designator designator))))

(lib:intern prelude "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (lib:un-char
        (prelude:read-char stream error-eofp eof-value)
        stream))
     (prelude:%read-stream-designator designator))))

(lib:intern prelude "unread-char"
   (:lambda (ch designator)
     (:if (prelude:charp ch)
          (lib:un-char ch (prelude:%write-stream-designator designator))
          (prelude:raise ch 'prelude:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(lib:intern prelude "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (lib:eq prelude:%eof% form)
                (:if eof-error
                     (prelude:raise stream 'prelude:read "early end of file")
                     eof-value)
                (:if (lib:eq form prelude:%read-list-eol%)
                     (prelude:raise stream 'prelude:read "unmatched close parenthesis")
                     form)))
         (prelude:%read stream)))
      (prelude:%read-stream-designator designator))))

(lib:intern prelude "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (lib:eq value prelude:%eof%)
                eof-value
                (lib:get-str line)))              
         (lib:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (lib:eq ch prelude:%eof%)
                    prelude:%eof%
                    (:if (lib:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (lib:wr-char ch line)
                            (prelude:null loop))))))
             (prelude:read-char stream eof-error-p prelude:%eof%)))
         ())))
      (lib:open :string :output ""))))

(lib:intern prelude "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (prelude:%andf (prelude:vectorp object) (prelude:%prelude-type-p object))
               (prelude:%vector-write object escape stream)
               (lib:write object escape stream)))
      (prelude:%read-stream-designator designator))))
