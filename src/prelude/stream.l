;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;
(mu:intern :prelude "streamp" (:lambda (t) (mu:eq :stream (mu:type-of t))))

;;;
;;; utilities
;;;
(mu:intern :prelude "%write-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         mu:std-out
         (:if (prelude:streamp designator)
              designator
              (prelude:raise designator
                          'prelude:%write-stream-designator
                          "not a stream designator")))))

(mu:intern :prelude "%read-stream-designator"
  (:lambda (designator)
    (:if (prelude:null designator)
         mu:std-in
         (:if (mu:eq :t designator)
              mu:std-in
              (:if (prelude:streamp designator)
                   designator
                   (prelude:raise designator
                               'prelude:%read-stream-designator
                               "not a stream designator"))))))

;;;
;;; constructors
;;;
(mu:intern :prelude "make-string-stream"
   (:lambda (dir init)
      (:if (prelude:keywordp dir)
           (:if (prelude:stringp init)
                (:if (mu:eq dir :input)
                     (mu:open :string :input init)
                     (:if (mu:eq dir :output)
                          (mu:open :string :output init)
                          (:if (mu:eq dir :bidir)
                               (mu:open :string :bidir init)
                               (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))))
                (prelude:raise dir 'prelude:make-string-stream "not a direction keyword"))
           (prelude:raise init 'prelude:make-string-stream "not a string initializer"))))

(mu:intern :prelude "open"
  (:lambda (dir path)
     (:if (prelude:stringp path)
          (:if (prelude:keywordp dir)
               (:if (mu:eq dir :input)
                    (mu:open :file :input path)
                    (:if (mu:eq dir :output)
                         (mu:open :file :output path)
                         (prelude:raise dir 'prelude:open "not a direction keyword")))
               (prelude:raise path 'prelude:open "not a path string"))
          (prelude:raise dir 'prelude:open "not a direction keyword"))))

(mu:intern :prelude "close"
   (:lambda (stream)
      (:if (prelude:streamp stream)
           (mu:close stream)
           (prelude:raise stream 'prelude:close "not a stream"))))

(mu:intern :prelude "with-open-stream"
   (:lambda (type dir specifier fn)
     (:if (prelude:functionp fn)
          (:if (prelude:stringp specifier)
               (:if (prelude:%orf (mu:eq :file type)
                               (mu:eq :string type))
                    (:if (prelude:%orf (mu:eq :input dir)
                                    (mu:eq :output dir))
                         ((:lambda (stream)
                             (mu:with-ex
                              (:lambda (ex) (mu:close stream) ex)
                              (:lambda () (prelude:apply fn `(,stream)))))
                          (mu:open type dir specifier))
                         (prelude:raise dir 'prelude:with-open-stream "not a direction keyword"))
                    (prelude:raise type 'prelude:with-open-stream "not a type keyword"))
               (prelude:raise path 'prelude:with-open-stream "not a stream specifier"))
          (prelude:raise dir 'prelude:with-open-stream "not a function"))))

;;;
;;; chars, bytes, and unread
;;;
(mu:intern :prelude "write-char"
   (:lambda (ch designator)
     (mu:wr-char ch (prelude:%write-stream-designator designator))))

(mu:intern :prelude "write-byte"
   (:lambda (byte designator)
     (mu:wr-byte byte (prelude:%write-stream-designator designator))))

(mu:intern :prelude "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:rd-char stream error-eofp eof-value))
     (prelude:%read-stream-designator designator))))

(mu:intern :prelude "read-byte"
   (:lambda (designator error-eofp eof-value)
     ((:lambda (stream)
        (mu:rd-byte stream error-eofp eof-value))
      (prelude:%read-stream-designator designator))))

(mu:intern :prelude "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (mu:un-char
        (prelude:read-char stream error-eofp eof-value)
        stream))
     (prelude:%read-stream-designator designator))))

(mu:intern :prelude "unread-char"
   (:lambda (ch designator)
     (:if (prelude:charp ch)
          (mu:un-char ch (prelude:%write-stream-designator designator))
          (prelude:raise ch 'prelude:unread-char "not a char"))))

;;;
;;; read/read-line/write
;;;
(mu:intern :prelude "read"
   (:lambda (designator eof-error eof-value)
     ((:lambda (stream)
        ((:lambda (form)
           (:if (mu:eq prelude:%eof% form)
                (:if eof-error
                     (prelude:raise stream 'prelude:read "early end of file")
                     eof-value)
                (:if (mu:eq form prelude:%read-list-eol%)
                     (prelude:raise stream 'prelude:read "unmatched close parenthesis")
                     form)))
         (prelude:%read stream)))
      (prelude:%read-stream-designator designator))))

(mu:intern :prelude "read-line"
   (:lambda (stream eof-error-p eof-value)
     ((:lambda (line)
        ((:lambda (value)
           (:if (mu:eq value prelude:%eof%)
                eof-value
                (mu:get-str line)))              
         (mu:fix
          (:lambda (loop)
            ((:lambda (ch)
               (:if (mu:eq ch prelude:%eof%)
                    prelude:%eof%
                    (:if (mu:eq #\linefeed ch)
                         loop
                         ((:lambda ()
                            (mu:wr-char ch line)
                            (prelude:null loop))))))
             (prelude:read-char stream eof-error-p prelude:%eof%)))
         ())))
      (mu:open :string :output ""))))

(mu:intern :prelude "write"
   (:lambda (object escape designator)
      ((:lambda (stream)
          (:if (prelude:%andf (prelude:vectorp object) (prelude:%prelude-type-p object))
               (prelude:%vector-write object escape stream)
               (mu:write object escape stream)))
      (prelude:%read-stream-designator designator))))
