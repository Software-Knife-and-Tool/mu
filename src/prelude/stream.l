;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;
(libcore:intern :prelude "streamp" (:lambda (t) (libcore:eq :stream (libcore:type-of t))))

;;;
;;; utilities
;;;
(libcore:intern :prelude "%write-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         libcore:std-out
         (:if (prelude:streamp designator)
              designator
              (prelude:raise designator
                          'prelude:%write-stream-designator
                          "not a stream designator")))))

(libcore:intern :prelude "%read-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         libcore:std-in
         (:if (libcore:eq :t designator)
              libcore:std-in
              (:if (prelude:streamp designator)
                   designator
                   (prelude:raise designator
                               'prelude:%read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(libcore:intern :prelude "make-string-stream"
   (:lambda (dir init)
      (:if (prelude:keywordp dir)
           (:if (prelude:stringp init)
                (:if (libcore:eq dir :input)
                     (libcore:open :string :input init)
                     (:if (libcore:eq dir :output)
                          (libcore:open :string :output init)
                          (:if (libcore:eq dir :bidir)
                               (libcore:open :string :bidir init)
                               (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))))
                (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))
           (prelude:raise init 'prelude:make-string-stream "not a string initializer"))))

(libcore:intern :prelude "open"
  (:lambda (dir path)
     (:if (prelude:stringp path)
          (:if (prelude:keywordp dir)
               (:if (libcore:eq dir :input)
                    (libcore:open :file :input path)
                    (:if (libcore:eq dir :output)
                         (libcore:open :file :output path)
                         (prelude:raise dir 'prelude:open "not a direction keyword")))
               (prelude:raise path 'prelude:open "not a path string"))
          (prelude:raise dir 'prelude:open "not a direction keyword"))))

(libcore:intern :prelude "close"
   (:lambda (stream)
      (:if (prelude:streamp stream)
           (libcore:close stream)
           (prelude:raise stream 'prelude:close "not a stream"))))

(libcore:intern :prelude "with-open-stream"
   (:lambda (type dir specifier fn)
     (:if (prelude:functionp fn)
          (:if (prelude:stringp specifier)
               (:if (prelude:%orf (libcore:eq :file type)
                               (libcore:eq :string type))
                    (:if (prelude:%orf (libcore:eq :input dir)
                                    (libcore:eq :output dir))
                         ((:lambda (stream)
                             (libcore:with-ex
                              (:lambda (ex) (libcore:close stream) ex)
                              (:lambda () (prelude:apply fn `(,stream)))))
                          (libcore:open type dir specifier))
                         (prelude:raise dir 'prelude:with-open-stream "not a direction keyword"))
                    (prelude:raise type 'prelude:with-open-stream "not a type keyword"))
               (prelude:raise path 'prelude:with-open-stream "not a stream specifier"))
          (prelude:raise dir 'prelude:with-open-stream "not a function"))))

;;;
;;; chars, bytes, and unread
;;;
(libcore:intern :prelude "write-char"
   (:lambda (ch designator)
     (libcore:wr-char ch (prelude:%write-stream-designator designator))))

(libcore:intern :prelude "write-byte"
   (:lambda (byte designator)
     (libcore:wr-byte byte (prelude:%write-stream-designator designator))))

(libcore:intern :prelude "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (libcore:rd-char stream error-eofp eof-value))
     (prelude:%read-stream-designator designator))))

(libcore:intern :prelude "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (libcore:rd-byte stream error-eofp eof-value))
      (prelude:%read-stream-designator designator))))

(libcore:intern :prelude "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (libcore:un-char
        (prelude:read-char stream error-eofp eof-value)
        stream))
     (prelude:%read-stream-designator designator))))

(libcore:intern :prelude "unread-char"
   (:lambda (ch designator)
     (:if (prelude:charp ch)
          (libcore:un-char ch (prelude:%write-stream-designator designator))
          (prelude:raise ch 'prelude:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(libcore:intern :prelude "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (libcore:eq prelude:%eof% form)
                (:if eof-error
                     (prelude:raise stream 'prelude:read "early end of file")
                     eof-value)
                (:if (libcore:eq form prelude:%read-list-eol%)
                     (prelude:raise stream 'prelude:read "unmatched close parenthesis")
                     form)))
         (prelude:%read stream)))
      (prelude:%read-stream-designator designator))))

(libcore:intern :prelude "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (libcore:eq value prelude:%eof%)
                eof-value
                (libcore:get-str line)))              
         (libcore:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (libcore:eq ch prelude:%eof%)
                    prelude:%eof%
                    (:if (libcore:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (libcore:wr-char ch line)
                            (prelude:null loop))))))
             (prelude:read-char stream eof-error-p prelude:%eof%)))
         ())))
      (libcore:open :string :output ""))))

(libcore:intern :prelude "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (prelude:%andf (prelude:vectorp object) (prelude:%prelude-type-p object))
               (prelude:%vector-write object escape stream)
               (libcore:write object escape stream)))
      (prelude:%read-stream-designator designator))))
