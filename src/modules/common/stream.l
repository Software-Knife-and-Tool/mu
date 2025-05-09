;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; streams
;;;

;;;
;;; implementation
;;;
(mu:intern common "%write-stream-designator"
  (:lambda (designator)
    (:if (core:null designator)
         mu:*standard-output*
         designator)))
         
(mu:intern common "%read-stream-designator"
  (:lambda (designator)
    (:if (core:null designator)
         mu:*standard-input*
         (:if (mu:eq :t designator)
              mu:*standard-input*
              designator))))

;;;
;;; constructors
;;;
(mu:intern common "make-string-stream"
   (:lambda (dir init)
      (:if (core:keywordp dir)
           (:if (core:stringp init)
                (:if (mu:eq dir :input)
                     (mu:open :string :input init :t)
                     (:if (mu:eq dir :output)
                          (mu:open :string :output init :t)
                          (:if (mu:eq dir :bidir)
                               (mu:open :string :bidir init :t)
                               (core:error "make-string-stream: not a direction keyword, ~A~%" `(,dir)))))
                (common:raise dir 'common:make-string-stream "not a direction keyword"))
           (common:raise init 'common:make-string-stream "not a string initializer"))))

(mu:intern common "open"
  (:lambda (dir path)
     (:if (core:stringp path)
          (:if (core:keywordp dir)
               (:if (mu:eq dir :input)
                    (mu:open :file :input path :t)
                    (:if (mu:eq dir :output)
                         (mu:open :file :output path :t)
                         (core:error "open: not a direction keyword, ~A~%" `(,dir))))
               (core:error "open: not a path string, ~A~%" `(,path)))
          (core:error "open: not a direction keyword ~A~%" `(,dir)))))

;;;
;;; functions
;;;
(mu:intern common "close"
   (:lambda (stream)
      (:if (core:streamp stream)
           (mu:close stream)
           (core:error "close: not a stream, ~A~%" `(,stream)))))

(mu:intern common "with-open-file"
   (:lambda (type dir specifier fn)
     (:if (core:functionp fn)
          (:if (core:%stringp specifier)
               (:if (core:%orf (mu:eq :file type)
                               (mu:eq :string type))
                    (:if (core:%orf (mu:eq :input dir)
                                    (mu:eq :output dir))
                         ((:lambda (stream)
                             (mu:with-exception
                              (:lambda (ex) (mu:close stream) ex)
                              (:lambda () (core:apply fn `(,stream)))))
                          (mu:open type dir specifier :t))
                         (core:error "with-open-file: not a direction keyword, ~A~%" `(,dir)))
                    (core:error "with-open-file: not a type keyword, ~A~%" `(,type)))
               (core:error "with-open-file: not a stream specifier, ~A~%" `(,specifier)))
          (core:error "with-open-file: not a function ~A~%" `(,dir)))))

(mu:intern common "load-file"
   (:lambda (path)
     (common:with-open-file :file :input path
       (:lambda (stream)
         (mu:fix
             (:lambda (loop)
               (:if (mu:eq form core:%eof%)
                    loop
                    (common:null loop))
               (mu:eval (core:compile form ())))))
       (core:read stream () core:%eof%))))

;;;
;;; read/write chars, bytes, and unread
;;;
(mu:intern common "write-char"
   (:lambda (ch designator)
     ((:lambda (stream)
        (:if (core:streamp stream)
             (mu:write-char ch stream)
             (core:error "write-char: not a stream designator" `(,stream))))
      (common:%write-stream-designator designator))))

(mu:intern common "write-byte"
   (:lambda (ch designator)
     ((:lambda (stream)
        (:if (core:streamp stream)
             (mu:write-byte ch stream)
             (core:error "write-byte: not a stream designator" `(,stream))))
      (common:%write-stream-designator designator))))

(mu:intern common "read-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (:if (core:streamp stream)
            (mu:read-char stream error-eofp eof-value)
            (core:error "read-char: not a stream designator" `(,stream))))
     (common:%read-stream-designator designator))))

(mu:intern common "read-byte"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (:if (core:streamp stream)
            (mu:read-byte stream error-eofp eof-value)
            (core:error "read-byte: not a stream designator" `(,stream))))
     (common:%read-stream-designator designator))))

(mu:intern common "peek-char"
  (:lambda (designator error-eofp eof-value)
    ((:lambda (stream)
       (:if (core:streamp stream)
            (mu:unread-char
             (common:read-char stream error-eofp eof-value)
             stream)
            (core:error "peek-char: not a stream designator" `(,stream))))
     (common:%read-stream-designator designator))))

(mu:intern common "unread-char"
   (:lambda (ch designator)
     (:if (common:charp ch)
          ((:lambda (stream)
             (mu:unread-char ch stream))
           (common:%write-stream-designator designator))
          (core:error "unread-char: not a char, ~A~%" `(,ch)))))

(mu:intern common "read-line"
   (:lambda (designator eof-error-p eof-value)
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
             (common:read-char designator eof-error-p core:%eof%)))
         ())))
      (mu:open :string :output "" :t))))
